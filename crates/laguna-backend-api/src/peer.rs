use crate::error::peer::PeerError;
use crate::error::APIError;

use actix_web::http::header::USER_AGENT;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use laguna_backend_dto::user::UserDTO;

use laguna_backend_model::peer::Peer;
use laguna_backend_tracker::http::announce::AnnounceRequest;

use laguna_backend_tracker_common::announce::AnnounceEvent;
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::PgPool;

use std::str::FromStr;

/// GET `/api/peer/announce`
pub async fn peer_announce(
  req: HttpRequest,
  announce_data: web::Query<AnnounceRequest>,
  pool: web::Data<PgPool>,
  user: UserDTO,
) -> Result<HttpResponse, APIError> {
  let maybe_peer = sqlx::query_as::<_, Peer>("SELECT * FROM peer_get($1)")
    .bind(&announce_data.peer_id)
    .fetch_optional(pool.get_ref())
    .await?;

  if let Some(peer) = maybe_peer {
    // We already know this peer, checkup on it
    return handle_peer_request(peer, announce_data.into_inner()).await;
  }

  let ip = announce_data.ip.map(IpNetwork::from).or_else(|| {
    req
      .connection_info()
      .realip_remote_addr() // go over proxy (if it exists)
      .and_then(|maybe_ip| IpNetwork::from_str(maybe_ip).ok())
  });

  let user_agent = req
    .headers()
    .get(USER_AGENT)
    .map(|hv| hv.to_str().expect("Cannot convert header value to str"));

  let peer =
    sqlx::query_as::<_, Peer>("SELECT * FROM peer_insert($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
      .bind(&announce_data.peer_id)
      .bind(&announce_data.info_hash)
      .bind(ip)
      .bind(announce_data.port as i32)
      .bind(user_agent)
      .bind(announce_data.uploaded)
      .bind(announce_data.downloaded)
      .bind(announce_data.left)
      .bind(Utc::now())
      .bind(user.id)
      .fetch_optional(pool.get_ref())
      .await?
      .ok_or_else(|| PeerError::DidntCreate)?;

  handle_peer_request(peer, announce_data.into_inner()).await
}

async fn handle_peer_request(
  peer: Peer,
  announce_data: AnnounceRequest,
) -> Result<HttpResponse, APIError> {
  let event = announce_data.event.unwrap_or_else(|| AnnounceEvent::Empty);
  match event {
    AnnounceEvent::Started => handle_peer_started(peer, announce_data).await,
    AnnounceEvent::Completed => handle_peer_completed(peer, announce_data).await,
    AnnounceEvent::Stopped => handle_peer_stopped(peer, announce_data).await,
    AnnounceEvent::Empty => Ok(HttpResponse::UnprocessableEntity().finish()),
  }
}

async fn handle_peer_started(
  _peer: Peer,
  _announce_data: AnnounceRequest,
) -> Result<HttpResponse, APIError> {
  Ok(HttpResponse::Ok().finish())
}

async fn handle_peer_stopped(
  _peer: Peer,
  _announce_data: AnnounceRequest,
) -> Result<HttpResponse, APIError> {
  Ok(HttpResponse::Ok().finish())
}

async fn handle_peer_completed(
  _peer: Peer,
  _announce_data: AnnounceRequest,
) -> Result<HttpResponse, APIError> {
  Ok(HttpResponse::Ok().finish())
}
