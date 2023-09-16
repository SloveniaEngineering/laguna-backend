use crate::error::peer::PeerError;

use actix_web::dev::PeerAddr;
use actix_web::http::header::USER_AGENT;
use actix_web::{web, HttpRequest, HttpResponse};

use bendy::encoding::ToBencode;
use chrono::Utc;
use laguna_backend_model::peer::Peer;

use laguna_backend_model::speedlevel::SpeedLevel;
use laguna_backend_model::torrent::Torrent;
use laguna_backend_tracker::http::announce::{Announce, AnnounceReply};

use laguna_backend_model::genre::Genre;
use laguna_backend_tracker_common::announce::AnnounceEvent;

use laguna_backend_model::behaviour::Behaviour;
use laguna_backend_tracker_common::peer::{PeerBin, PeerDict, PeerStream};
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::PgPool;

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

#[utoipa::path(
  get,
  path = "/peer/announce",
  responses((status = 200, body = String, description = "Returns bencoded `AnnounceReply`", content_type = "text/plain"))
)]
pub async fn peer_announce<const N: usize>(
  peer_addr: PeerAddr,
  req: HttpRequest,
  announce_data: web::Query<Announce<N>>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, PeerError<N>> {
  sqlx::query_file_as!(
    Torrent,
    "queries/torrent_get.sql",
    announce_data.info_hash as _
  )
  .fetch_optional(pool.get_ref())
  .await?
  .map(drop)
  .ok_or(PeerError::UnknownTorrent(announce_data.info_hash.clone()))?;

  let maybe_peer = sqlx::query_file_as!(Peer, "queries/peer_get.sql", announce_data.peer_id as _)
    .fetch_optional(pool.get_ref())
    .await?;

  handle_peer_request(
    req,
    maybe_peer,
    announce_data.into_inner(),
    pool,
    peer_addr.0,
  )
  .await
}

async fn handle_peer_request<const N: usize>(
  req: HttpRequest,
  maybe_peer: Option<Peer>,
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
  peer_addr: SocketAddr,
) -> Result<HttpResponse, PeerError<N>> {
  let event = announce_data.event.unwrap_or(AnnounceEvent::Empty);
  match (event, maybe_peer) {
    (AnnounceEvent::Started, Some(_)) => {
      // We already know this peer but it sent a started event.
      // Treat it as an update.
      log::info!(
        "Peer {} sent started event, treating as update.",
        announce_data.peer_id
      );
      handle_peer_updated(announce_data, pool).await
    },
    (AnnounceEvent::Started, None) => {
      log::info!(
        "Peer {} sent started event, treating as start.",
        announce_data.peer_id
      );
      handle_peer_started(req, announce_data, pool, peer_addr).await
    },
    (AnnounceEvent::Completed, Some(_)) => {
      log::info!("Peer {} sent completed event.", announce_data.peer_id);
      handle_peer_completed(announce_data, pool).await
    },
    (AnnounceEvent::Completed, None) => Err(PeerError::<N>::UnexpectedEvent {
      event: AnnounceEvent::Completed,
      message: String::from("Inexistant peer sent completion."),
    }),
    (AnnounceEvent::Stopped, Some(peer)) => handle_peer_stopped(peer, announce_data, pool).await,
    (AnnounceEvent::Stopped, None) => Err(PeerError::<N>::UnexpectedEvent {
      event: AnnounceEvent::Stopped,
      message: String::from("Inexistant peer sent stop."),
    }),
    (AnnounceEvent::Updated, Some(_)) => handle_peer_updated(announce_data, pool).await,
    (AnnounceEvent::Updated, None) => Err(PeerError::<N>::UnexpectedEvent {
      event: AnnounceEvent::Updated,
      message: String::from("Inexistant peer sent update."),
    }),
    (AnnounceEvent::Paused, Some(peer)) => handle_peer_paused(peer, announce_data, pool).await,
    (AnnounceEvent::Paused, None) => Err(PeerError::<N>::UnexpectedEvent {
      event: AnnounceEvent::Paused,
      message: String::from("Inexistant peer sent pause."),
    }),
    (AnnounceEvent::Empty, Some(_)) => {
      // Empty event with a peer, assume it's an update (as per BitTorrent spec).
      log::info!(
        "Peer {} sent empty event, treating as update.",
        announce_data.peer_id
      );
      handle_peer_updated(announce_data, pool).await
    },
    (AnnounceEvent::Empty, None) => {
      // Empty event with no peer, assume it's a start.
      log::info!(
        "Peer {} sent empty event, treating as start.",
        announce_data.peer_id
      );
      handle_peer_started(req, announce_data, pool, peer_addr).await
    },
  }
}

async fn handle_peer_started<const N: usize>(
  req: HttpRequest,
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
  peer_addr: SocketAddr,
) -> Result<HttpResponse, PeerError<N>> {
  // If ip was specified by client, prefer it over the one in the request.
  // If proxy is used, prefer the original ip.
  let ip = IpNetwork::from(if let Some(ip) = announce_data.ip {
    ip
  } else if let Some(ip) = req.connection_info().realip_remote_addr() {
    // Forwarded by proxy
    if let Ok(ip) = IpAddr::from_str(ip) {
      ip
    } else {
      peer_addr.ip()
    }
  } else {
    peer_addr.ip()
  });

  let user_agent = req
    .headers()
    .get(USER_AGENT)
    .map(|hv| hv.to_str().expect("Cannot convert header value to str"));

  let swarm = torrent_swarm(pool.get_ref(), &announce_data).await?;

  let behaviour = if announce_data.left == 0 {
    Behaviour::Seed
  } else {
    Behaviour::Downloader
  };

  sqlx::query_file_as!(
    Peer,
    "queries/peer_insert.sql",
    announce_data.peer_id as _,
    None::<String>,
    announce_data.info_hash as _,
    ip,
    announce_data.port as i32,
    swarm.is_empty(), // if we are the first peer, we are origin
    user_agent,
    announce_data.uploaded,
    announce_data.downloaded,
    announce_data.left,
    behaviour as _,
    Utc::now(),
    Utc::now(),
  )
  .fetch_optional(pool.get_ref())
  .await?
  .map(drop)
  .ok_or(PeerError::NotCreated)?;

  let (complete, incomplete) = complete_incomplete_counts(&swarm).await;

  Ok(
    HttpResponse::Ok().body(
      AnnounceReply {
        failure_reason: None,
        warning_message: None,
        complete,
        incomplete,
        tracker_id: None,
        min_interval: None,
        interval: 1,
        peers: peer_stream(announce_data.compact.unwrap_or(true), swarm),
      }
      .to_bencode()?,
    ),
  )
}

async fn handle_peer_stopped<const N: usize>(
  peer: Peer,
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, PeerError<N>> {
  sqlx::query_file_as!(
    Peer,
    "queries/peer_update.sql",
    announce_data.uploaded,
    announce_data.downloaded,
    announce_data.left,
    Behaviour::Stopped as _,
    Utc::now(),
    announce_data.peer_id as _
  )
  .fetch_optional(pool.get_ref())
  .await?
  .ok_or(PeerError::NotUpdated)?;
  Ok(
    HttpResponse::Ok().body(
      AnnounceReply {
        failure_reason: None,
        warning_message: None,
        complete: 0,
        incomplete: 0,
        tracker_id: None,
        min_interval: None,
        interval: 1,
        peers: peer_stream(announce_data.compact.unwrap_or(true), vec![]),
      }
      .to_bencode()?,
    ),
  )
}

async fn handle_peer_completed<const N: usize>(
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, PeerError<N>> {
  let swarm = torrent_swarm(pool.get_ref(), &announce_data).await?;
  let (complete, incomplete) = complete_incomplete_counts(&swarm).await;
  Ok(
    HttpResponse::Ok().body(
      AnnounceReply {
        failure_reason: None,
        warning_message: None,
        complete,
        incomplete,
        tracker_id: None,
        min_interval: None,
        interval: 1,
        peers: peer_stream(announce_data.compact.unwrap_or(true), swarm),
      }
      .to_bencode()?,
    ),
  )
}

async fn handle_peer_updated<const N: usize>(
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, PeerError<N>> {
  let swarm = torrent_swarm(pool.get_ref(), &announce_data).await?;

  let behaviour = if announce_data.left == 0 {
    Behaviour::Seed
  } else {
    Behaviour::Downloader
  };

  sqlx::query_file_as!(
    Peer,
    "queries/peer_update.sql",
    announce_data.uploaded,
    announce_data.downloaded,
    announce_data.left,
    behaviour as _,
    Utc::now(),
    announce_data.peer_id as _
  )
  .fetch_optional(pool.get_ref())
  .await?
  .map(drop)
  .ok_or(PeerError::NotUpdated)?;

  let (complete, incomplete) = complete_incomplete_counts(&swarm).await;

  Ok(
    HttpResponse::Ok().body(
      AnnounceReply {
        failure_reason: None,
        warning_message: None,
        complete,
        incomplete,
        tracker_id: None,
        min_interval: None,
        interval: 1,
        peers: peer_stream(announce_data.compact.unwrap_or(true), swarm),
      }
      .to_bencode()?,
    ),
  )
}

async fn handle_peer_paused<const N: usize>(
  _peer: Peer,
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, PeerError<N>> {
  let swarm = torrent_swarm(pool.get_ref(), &announce_data).await?;
  let (complete, incomplete) = complete_incomplete_counts(&swarm).await;

  Ok(
    HttpResponse::Ok().body(
      AnnounceReply {
        failure_reason: None,
        warning_message: None,
        complete,
        incomplete,
        tracker_id: None,
        min_interval: None,
        interval: 1,
        peers: peer_stream(announce_data.compact.unwrap_or(true), swarm),
      }
      .to_bencode()?,
    ),
  )
}

#[inline]
async fn torrent_swarm<const N: usize>(
  pool: &PgPool,
  announce_data: &Announce<N>,
) -> Result<Vec<Peer>, PeerError<N>> {
  Ok(
    sqlx::query_file_as!(
      Peer,
      "queries/torrent_swarm_noself.sql",
      &announce_data.info_hash as _,
      &announce_data.peer_id as _
    )
    .fetch_all(pool)
    .await?,
  )
}

async fn complete_incomplete_counts(torrent_swarm: &Vec<Peer>) -> (u64, u64) {
  torrent_swarm
    .iter()
    .fold((0, 0), |(complete, incomplete), peer| {
      match peer.left_bytes {
        0 => (complete + 1, incomplete),
        _ => (complete, incomplete + 1),
      }
    })
}

fn peer_stream<'a>(compact: bool, torrent_swarm: Vec<Peer>) -> PeerStream {
  if !compact || torrent_swarm.iter().any(|peer| peer.ip.is_ipv6()) {
    PeerStream::Dict(
      torrent_swarm
        .into_iter()
        .map(|peer| PeerDict {
          peer_id: peer.id,
          ip: peer.ip.ip(),
          port: peer.port as u16,
        })
        .collect::<Vec<PeerDict>>(),
    )
  } else {
    PeerStream::Bin(
      torrent_swarm
        .into_iter()
        .flat_map(|peer| PeerBin::from_socket(peer.ip.ip(), peer.port as u16).0)
        .collect::<Vec<u8>>(),
    )
  }
}
