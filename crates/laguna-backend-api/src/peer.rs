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

#[derive(Debug, sqlx::FromRow)]
struct PeerCount {
  complete: Option<i64>,
  incomplete: Option<i64>,
}

#[allow(missing_docs)]
#[utoipa::path(
  get,
  path = "/peer/announce",
  responses((status = 200, body = String, description = "Returns bencoded `AnnounceReply`", content_type = "text/plain"))
)]
pub async fn peer_announce<const N: usize>(
  peer_addr: PeerAddr,
  req: HttpRequest,
  announce: web::Query<Announce<N>>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, PeerError<N>> {
  let ip = IpNetwork::from(peer_addr.into_inner().ip());

  let user_agent = req
    .headers()
    .get(USER_AGENT)
    .map(|hv| hv.to_str().expect("Cannot convert header value to str"));

  let download = sqlx::query_file_as!(
    Download::<N>,
    "queries/download_lookup_byhash.sql",
    announce.down_hash as _
  )
  .fetch_optional(pool.get_ref())
  .await?
  .ok_or(PeerError::DownloadNotFound(announce.down_hash.clone()))?;

  let user = sqlx::query_file_as!(User, "queries/user_get.sql", download.user_id as _)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or(PeerError::UnknownUser(download.user_id))?;

  // Check if torrent exists on tracker
  sqlx::query_file_as!(Torrent, "queries/torrent_get.sql", announce.info_hash as _)
    .fetch_optional(pool.get_ref())
    .await?
    .map(drop::<Torrent<N>>)
    .ok_or(PeerError::UnknownTorrent(announce.info_hash.clone()))?;

  let maybe_peer = sqlx::query_file_as!(Peer::<N>, "queries/peer_get.sql", announce.peer_id as _)
    .fetch_optional(pool.get_ref())
    .await?;

  let behaviour = if announce.left == 0 {
    Behaviour::Seed
  } else {
    Behaviour::Downloader
  };

  let swarm = sqlx::query_file_as!(
    Peer::<N>,
    "queries/torrent_swarm_noself.sql",
    &announce.info_hash as _,
    &announce.peer_id as _
  )
  .fetch_all(pool.get_ref())
  .await
  .map_err(PeerError::from)?;

  let peer_count = sqlx::query_file_as!(
    PeerCount,
    "queries/torrent_swarm_complete_incomplete_noself.sql",
    &announce.info_hash as _,
    &announce.peer_id as _
  )
  .fetch_one(pool.get_ref())
  .await
  .map_err(PeerError::from)?;

  handle_peer_request(
    maybe_peer,
    announce.into_inner(),
    user,
    pool,
    swarm,
    peer_count,
    user_agent,
    ip,
    behaviour,
  )
  .await
}

/// Delegates peer request to one of sub-functions depend on event type.
#[allow(clippy::too_many_arguments)]
async fn handle_peer_request<const N: usize>(
  maybe_peer: Option<Peer<N>>,
  announce_data: Announce<N>,
  user: User,
  pool: web::Data<PgPool>,
  swarm: Vec<Peer<N>>,
  peer_count: PeerCount,
  user_agent: Option<&str>,
  ip: IpNetwork,
  behaviour: Behaviour,
) -> Result<HttpResponse, PeerError<N>> {
  match (announce_data.event, maybe_peer) {
    (Some(AnnounceEvent::Started), Some(_)) => {
      // We already know this peer but it sent a started event.
      // Treat it as an update.
      handle_peer_updated(announce_data, pool, swarm, behaviour).await
    },
    (Some(AnnounceEvent::Started), None) => {
      handle_peer_started(announce_data, user, pool, swarm, user_agent, ip, behaviour).await
    },
    (Some(AnnounceEvent::Completed), Some(_)) => {
      handle_peer_completed(announce_data, pool, swarm, peer_count, behaviour).await
    },
    (Some(AnnounceEvent::Stopped), Some(_)) => handle_peer_stopped(announce_data, pool).await,
    (Some(AnnounceEvent::Paused), Some(_)) => {
      handle_peer_paused(announce_data, pool, swarm, peer_count).await
    },
    (Some(AnnounceEvent::Updated), Some(_)) => {
      handle_peer_updated(announce_data, pool, swarm, behaviour).await
    },
    (Some(event), None) => Err(PeerError::UnknownPeerSentEvent(event)),
    (None, Some(_)) => {
      // Empty event with a peer, assume it's an update (as per BitTorrent spec).
      handle_peer_updated(announce_data, pool, swarm, behaviour).await
    },
    (None, None) => {
      // Empty event with no peer, assume it's a start.
      handle_peer_started(announce_data, user, pool, swarm, user_agent, ip, behaviour).await
    },
  }
}

#[allow(missing_docs)]
async fn handle_peer_started<const N: usize>(
  announce_data: Announce<N>,
  user: User,
  pool: web::Data<PgPool>,
  swarm: Vec<Peer<N>>,
  user_agent: Option<&str>,
  ip: IpNetwork,
  behaviour: Behaviour,
) -> Result<HttpResponse, PeerError<N>> {
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

  let peer_count = sqlx::query_file_as!(
    PeerCount,
    "queries/torrent_swarm_complete_incomplete_noself.sql",
    &announce_data.info_hash as _,
    &announce_data.peer_id as _
  )
  .fetch_one(pool.get_ref())
  .await
  .map_err(PeerError::from)?;

  Ok(
    HttpResponse::Ok().body(
      AnnounceReply::success(
        peer_count.complete.unwrap_or_default() as u64,
        peer_count.incomplete.unwrap_or_default() as u64,
        peer_stream(
          announce_data.compact.unwrap_or(true),
          announce_data.no_peer_id.unwrap_or_default(),
          swarm,
          behaviour,
        ),
      )
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
      AnnounceReply::success(
        0,
        0,
        peer_stream::<N>(
          announce_data.compact.unwrap_or(true),
          announce_data.no_peer_id.unwrap_or_default(),
          Vec::new(),
          Behaviour::Stopped,
        ),
      )
      .to_bencode()?,
    ),
  )
}

#[allow(missing_docs)]
async fn handle_peer_completed<const N: usize>(
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
  swarm: Vec<Peer<N>>,
  peer_count: PeerCount,
  behaviour: Behaviour,
) -> Result<HttpResponse, PeerError<N>> {
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
  .ok_or(PeerError::NotUpdated)?;
  Ok(
    HttpResponse::Ok().body(
      AnnounceReply::success(
        peer_count.complete.unwrap_or_default() as u64,
        peer_count.incomplete.unwrap_or_default() as u64,
        peer_stream::<N>(
          announce_data.compact.unwrap_or(true),
          announce_data.no_peer_id.unwrap_or_default(),
          swarm,
          behaviour,
        ),
      )
      .to_bencode()?,
    ),
  )
}

#[allow(missing_docs)]
async fn handle_peer_updated<const N: usize>(
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
  swarm: Vec<Peer<N>>,
  behaviour: Behaviour,
) -> Result<HttpResponse, PeerError<N>> {
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

  let peer_count = sqlx::query_file_as!(
    PeerCount,
    "queries/torrent_swarm_complete_incomplete_noself.sql",
    &announce_data.info_hash as _,
    &announce_data.peer_id as _
  )
  .fetch_one(pool.get_ref())
  .await
  .map_err(PeerError::from)?;

  Ok(
    HttpResponse::Ok().body(
      AnnounceReply::success(
        peer_count.complete.unwrap_or_default() as u64,
        peer_count.incomplete.unwrap_or_default() as u64,
        peer_stream::<N>(
          announce_data.compact.unwrap_or(true),
          announce_data.no_peer_id.unwrap_or_default(),
          swarm,
          behaviour,
        ),
      )
      .to_bencode()?,
    ),
  )
}

#[allow(missing_docs)]
async fn handle_peer_paused<const N: usize>(
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
  swarm: Vec<Peer<N>>,
  peer_count: PeerCount,
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
  .map(drop)
  .ok_or(PeerError::NotUpdated)?;

  Ok(
    HttpResponse::Ok().body(
      AnnounceReply::success(
        peer_count.complete.unwrap_or_default() as u64,
        peer_count.incomplete.unwrap_or_default() as u64,
        peer_stream::<N>(
          announce_data.compact.unwrap_or(true),
          announce_data.no_peer_id.unwrap_or_default(),
          swarm,
          Behaviour::Stopped,
        ),
      )
      .to_bencode()?,
    ),
  )
}

/// Creates [`PeerStream`] based on `compact` and other parameters.
/// [`behaviour`] is the behaviour of current peer, if [`Behaviour::Seed`] we don't show other seeds to it.
fn peer_stream<const N: usize>(
  compact: bool,
  no_peer_id: bool,
  swarm: Vec<Peer<N>>,
  behaviour: Behaviour,
) -> PeerStream {
  if !compact || swarm.iter().any(|peer| peer.ip.is_ipv6()) {
    PeerStream::Dict(
      swarm
        .into_iter()
        .filter(|p| behaviour == Behaviour::Seed && p.behaviour != behaviour)
        .map(|peer| PeerDict {
          peer_id: if no_peer_id { None } else { Some(peer.id) },
          ip: peer.ip.ip(),
          port: peer.port as u16,
        })
        .collect::<Vec<PeerDict>>(),
    )
  } else {
    PeerStream::Bin(
      swarm
        .into_iter()
        .filter(|p| behaviour == Behaviour::Seed && p.behaviour != behaviour)
        .flat_map(|peer| PeerBin::from_socket(peer.ip.ip(), peer.port as u16).0)
        .collect::<Vec<u8>>(),
    )
  }
}
