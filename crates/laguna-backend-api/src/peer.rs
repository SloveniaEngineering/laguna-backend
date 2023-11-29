use crate::error::peer::PeerError;

use actix_web::dev::PeerAddr;
use actix_web::http::header::USER_AGENT;
use actix_web::{web, HttpRequest, HttpResponse};

use bendy::encoding::ToBencode;
use chrono::Utc;

use laguna_backend_model::peer::Peer;
use laguna_backend_model::user::User;

use laguna_backend_model::download::Download;
use laguna_backend_model::speedlevel::SpeedLevel;
use laguna_backend_model::torrent::Torrent;
use laguna_backend_tracker::http::announce::{Announce, AnnounceReply};

use laguna_backend_model::genre::Genre;
use laguna_backend_tracker_common::announce::AnnounceEvent;

use laguna_backend_model::behaviour::Behaviour;
use laguna_backend_model::role::Role;

use laguna_backend_tracker_common::peer::{PeerBin, PeerDict, PeerStream};
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::PgPool;

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

#[allow(missing_docs)]
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
  let download = sqlx::query_file_as!(
    Download::<N>,
    "queries/download_lookup_byhash.sql",
    announce_data.down_hash as _
  )
  .fetch_optional(pool.get_ref())
  .await?
  .ok_or(PeerError::DownloadNotFound(announce_data.down_hash.clone()))?;

  let user = sqlx::query_file_as!(User, "queries/user_get.sql", download.user_id as _)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or(PeerError::UnknownUser(download.user_id))?;

  // Check if torrent exists on tracker
  sqlx::query_file_as!(
    Torrent,
    "queries/torrent_get.sql",
    announce_data.info_hash as _
  )
  .fetch_optional(pool.get_ref())
  .await?
  .map(drop::<Torrent<N>>)
  .ok_or(PeerError::UnknownTorrent(announce_data.info_hash.clone()))?;

  let maybe_peer = sqlx::query_file_as!(
    Peer::<N>,
    "queries/peer_get.sql",
    announce_data.peer_id as _
  )
  .fetch_optional(pool.get_ref())
  .await?;

  handle_peer_request(
    req,
    maybe_peer,
    announce_data.into_inner(),
    user,
    pool,
    peer_addr.0,
  )
  .await
}

/// Delegates peer request to one of subfunctions depend on event type.
async fn handle_peer_request<const N: usize>(
  req: HttpRequest,
  maybe_peer: Option<Peer<N>>,
  announce_data: Announce<N>,
  user: User,
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
      handle_peer_started(req, announce_data, user, pool, peer_addr).await
    },
    (AnnounceEvent::Completed, Some(_)) => {
      log::info!("Peer {} sent completed event.", announce_data.peer_id);
      handle_peer_completed(announce_data, pool).await
    },
    (AnnounceEvent::Completed, None) => Err(PeerError::<N>::UnexpectedEvent {
      event: AnnounceEvent::Completed,
      message: String::from("Inexistant peer sent completion."),
    }),
    (AnnounceEvent::Stopped, Some(_peer)) => handle_peer_stopped(announce_data, pool).await,
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
      handle_peer_started(req, announce_data, user, pool, peer_addr).await
    },
  }
}

#[allow(missing_docs)]
async fn handle_peer_started<const N: usize>(
  req: HttpRequest,
  announce_data: Announce<N>,
  user: User,
  pool: web::Data<PgPool>,
  peer_addr: SocketAddr,
) -> Result<HttpResponse, PeerError<N>> {
  let ip = IpNetwork::from(peer_addr.ip());
  
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
    Peer::<N>,
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
    user.id
  )
  .fetch_optional(pool.get_ref())
  .await?
  .map(drop)
  .ok_or(PeerError::NotCreated)?;

  let peer_count = complete_incomplete_counts(pool.get_ref(), &announce_data).await?;

  Ok(
    HttpResponse::Ok().body(
      AnnounceReply {
        failure_reason: None,
        warning_message: None,
        complete: peer_count.complete.unwrap_or_default() as u64,
        incomplete: peer_count.incomplete.unwrap_or_default() as u64,
        tracker_id: None,
        min_interval: None,
        interval: 1,
        peers: peer_stream(
          announce_data.compact.unwrap_or(true),
          announce_data.no_peer_id.unwrap_or_default(),
          if behaviour == Behaviour::Seed {
            // report non-seeds to seed
            swarm
              .into_iter()
              .filter(|peer| peer.behaviour != Behaviour::Seed)
              .collect()
          } else {
            swarm
          },
        ),
      }
      .to_bencode()?,
    ),
  )
}

#[allow(missing_docs)]
async fn handle_peer_stopped<const N: usize>(
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, PeerError<N>> {
  sqlx::query_file_as!(
    Peer::<N>,
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
        peers: peer_stream::<N>(
          announce_data.compact.unwrap_or(true),
          announce_data.no_peer_id.unwrap_or_default(),
          vec![],
        ),
      }
      .to_bencode()?,
    ),
  )
}

#[allow(missing_docs)]
async fn handle_peer_completed<const N: usize>(
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, PeerError<N>> {
  let swarm = torrent_swarm(pool.get_ref(), &announce_data).await?;
  let peer_count = complete_incomplete_counts(pool.get_ref(), &announce_data).await?;
  Ok(
    HttpResponse::Ok().body(
      AnnounceReply {
        failure_reason: None,
        warning_message: None,
        complete: peer_count.complete.unwrap_or_default() as u64,
        incomplete: peer_count.incomplete.unwrap_or_default() as u64,
        tracker_id: None,
        min_interval: None,
        interval: 1,
        peers: peer_stream(
          announce_data.compact.unwrap_or(true),
          announce_data.no_peer_id.unwrap_or_default(),
          swarm,
        ),
      }
      .to_bencode()?,
    ),
  )
}

#[allow(missing_docs)]
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
    Peer::<N>,
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

  let peer_count = complete_incomplete_counts(pool.get_ref(), &announce_data).await?;

  Ok(
    HttpResponse::Ok().body(
      AnnounceReply {
        failure_reason: None,
        warning_message: None,
        complete: peer_count.complete.unwrap_or_default() as u64,
        incomplete: peer_count.incomplete.unwrap_or_default() as u64,
        tracker_id: None,
        min_interval: None,
        interval: 1,
        peers: peer_stream(
          announce_data.compact.unwrap_or(true),
          announce_data.no_peer_id.unwrap_or_default(),
          if behaviour == Behaviour::Seed {
            // report non-seeds to seed
            swarm
              .into_iter()
              .filter(|peer| peer.behaviour != Behaviour::Seed)
              .collect()
          } else {
            swarm
          },
        ),
      }
      .to_bencode()?,
    ),
  )
}

#[allow(missing_docs)]
async fn handle_peer_paused<const N: usize>(
  _peer: Peer<N>,
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, PeerError<N>> {
  let swarm = torrent_swarm(pool.get_ref(), &announce_data).await?;
  let peer_count = complete_incomplete_counts(pool.get_ref(), &announce_data).await?;

  Ok(
    HttpResponse::Ok().body(
      AnnounceReply {
        failure_reason: None,
        warning_message: None,
        complete: peer_count.complete.unwrap_or_default() as u64,
        incomplete: peer_count.incomplete.unwrap_or_default() as u64,
        tracker_id: None,
        min_interval: None,
        interval: 1,
        peers: peer_stream(
          announce_data.compact.unwrap_or(true),
          announce_data.no_peer_id.unwrap_or_default(),
          swarm,
        ),
      }
      .to_bencode()?,
    ),
  )
}

/// Returns all peers in a specific torrent's swarm excluding oneself.
#[inline]
async fn torrent_swarm<const N: usize>(
  pool: &PgPool,
  announce_data: &Announce<N>,
) -> Result<Vec<Peer<N>>, PeerError<N>> {
  sqlx::query_file_as!(
    Peer::<N>,
    "queries/torrent_swarm_noself.sql",
    &announce_data.info_hash as _,
    &announce_data.peer_id as _
  )
  .fetch_all(pool)
  .await
  .map_err(PeerError::from)
}

#[derive(Debug, sqlx::FromRow)]
struct PeerCount {
  complete: Option<i64>,
  incomplete: Option<i64>,
}

/// Returns how many peers in swarm completed or incompleted excluding oneself.
/// NOTE: This is not realtime but close enough.
async fn complete_incomplete_counts<const N: usize>(
  pool: &PgPool,
  announce_data: &Announce<N>,
) -> Result<PeerCount, PeerError<N>> {
  sqlx::query_file_as!(
    PeerCount,
    "queries/torrent_swarm_complete_incomplete_noself.sql",
    &announce_data.info_hash as _,
    &announce_data.peer_id as _
  )
  .fetch_one(pool)
  .await
  .map_err(PeerError::from)
}

/// Creates [`PeerStream`] based on `compact` and other parameters.
fn peer_stream<const N: usize>(
  compact: bool,
  no_peer_id: bool,
  torrent_swarm: Vec<Peer<N>>,
) -> PeerStream {
  if !compact || torrent_swarm.iter().any(|peer| peer.ip.is_ipv6()) {
    PeerStream::Dict(
      torrent_swarm
        .into_iter()
        .map(|peer| PeerDict {
          peer_id: if no_peer_id { None } else { Some(peer.id) },
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
