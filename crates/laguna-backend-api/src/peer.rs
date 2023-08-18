use crate::error::peer::PeerError;
use crate::error::APIError;

use actix_web::http::header::USER_AGENT;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use laguna_backend_dto::user::UserDTO;

use laguna_backend_model::behaviour::Behaviour;
use laguna_backend_model::peer::Peer;
use laguna_backend_tracker::http::announce::AnnounceRequest;

use laguna_backend_tracker_common::announce::AnnounceEvent;
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::PgPool;

use std::str::FromStr;

/// GET `/api/peer/announce`
/// # Example
/// ### Request
/// **NOTE**: The -G allows GET to send data via query string. See: <https://stackoverflow.com/questions/13371284/curl-command-line-url-parameters>.
///
/// **FIXME**: This example doesn't work yet, URL encoding info_hash and peer_id is weird ASF.
///            Send 20 bytes of info_hash and 20 bytes of peer_id via GET request.
///            Curl might be an issue here.
///
/// ```bash
/// # -G + -d 'neki=123' gives http://127.0.0.1:6969/api/peer/announce?neki=123
/// curl -X GET \
///      -G \
///      -i 'http://127.0.0.1:6969/api/peer/announce' \
///      -d 'info_hash=%5B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%2C13%2C14%2C15%2C16%2C17%2C18%2C19%2C20%5D' \
///      -d 'peer_id=%5B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%2C13%2C14%2C15%2C16%2C17%2C18%2C19%2C20%5D' \
///      -d 'ip=127.0.0.1' \
///      -d 'port=43222' \
///      -d 'uploaded=0' \
///      -d 'downloaded=0' \
///      -d 'left=0' \
///      -d 'event=started' \
///      -d 'numwant=10' \
///      -d 'compact=true' \
///      -d 'no_peer_id=true' \
///      -d 'key=some%20random%20key' \
///      -d 'trackerid=some%20prev%20tracker%20id' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2OTEzMjIyODksImlhdCI6MTY5MTIzNTg4OSwiaWQiOiJjZjNhNTRlOC01MmUzLTQ3OTktOTNmNS1jMGM0NjE3ZTUxNjYiLCJ1c2VybmFtZSI6InRlc3R4eHgiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDgtMDVUMTE6Mjc6MjQuNDY5NDM4WiIsImxhc3RfbG9naW4iOiIyMDIzLTA4LTA1VDExOjQ0OjQ5LjAyNzI3MFoiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.TDELyQha28VDGd0L5trCi8hiTxLESCmHUnLE9l7h2W4' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2OTEzMjIyODksImlhdCI6MTY5MTIzNTg4OSwiaWQiOiJjZjNhNTRlOC01MmUzLTQ3OTktOTNmNS1jMGM0NjE3ZTUxNjYiLCJ1c2VybmFtZSI6InRlc3R4eHgiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDgtMDVUMTE6Mjc6MjQuNDY5NDM4WiIsImxhc3RfbG9naW4iOiIyMDIzLTA4LTA1VDExOjQ0OjQ5LjAyNzI3MFoiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.TDELyQha28VDGd0L5trCi8hiTxLESCmHUnLE9l7h2W4' \
///      -H 'Content-Type: text/plain'
/// ```
/// ### Response
/// ```text
/// ```
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

  handle_peer_request(
    maybe_peer,
    announce_data.into_inner(),
    pool,
    ip,
    user_agent,
    user,
  )
  .await
}

async fn handle_peer_request(
  maybe_peer: Option<Peer>,
  announce_data: AnnounceRequest,
  pool: web::Data<PgPool>,
  ip: Option<IpNetwork>,
  user_agent: Option<&str>,
  user: UserDTO,
) -> Result<HttpResponse, APIError> {
  let event = announce_data.event.unwrap_or_else(|| AnnounceEvent::Empty);
  match event {
    AnnounceEvent::Started => match maybe_peer {
      Some(peer) => {
        // We already know this peer but it sent a started event.
        // Treat it as an update.
        handle_peer_updated(peer, announce_data, pool, ip, user_agent).await
      },
      None => handle_peer_started(announce_data, pool, ip, user_agent, user).await,
    },
    AnnounceEvent::Completed => match maybe_peer {
      Some(peer) => handle_peer_completed(peer, announce_data).await,
      None => Err(
        PeerError::UnexpectedEvent {
          event: AnnounceEvent::Completed,
          message: String::from("Inexistant peer sent completion."),
        }
        .into(),
      ),
    },
    AnnounceEvent::Stopped => match maybe_peer {
      Some(peer) => handle_peer_stopped(peer, announce_data).await,
      None => Err(
        PeerError::UnexpectedEvent {
          event: AnnounceEvent::Stopped,
          message: String::from("Inexistant peer sent stop."),
        }
        .into(),
      ),
    },
    AnnounceEvent::Updated => match maybe_peer {
      Some(peer) => handle_peer_updated(peer, announce_data, pool, ip, user_agent).await,
      None => Err(
        PeerError::UnexpectedEvent {
          event: AnnounceEvent::Updated,
          message: String::from("Inexistant peer sent update."),
        }
        .into(),
      ),
    },
    AnnounceEvent::Paused => match maybe_peer {
      Some(peer) => handle_peer_paused(peer, announce_data).await,
      None => Err(
        PeerError::UnexpectedEvent {
          event: AnnounceEvent::Paused,
          message: String::from("Inexistant peer sent pause."),
        }
        .into(),
      ),
    },
    AnnounceEvent::Empty => Ok(HttpResponse::UnprocessableEntity().finish()),
  }
}

async fn handle_peer_started(
  announce_data: AnnounceRequest,
  pool: web::Data<PgPool>,
  ip: Option<IpNetwork>,
  user_agent: Option<&str>,
  user: UserDTO,
) -> Result<HttpResponse, APIError> {
  let _peer =
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
      .ok_or_else(|| PeerError::NotCreated)?;
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

async fn handle_peer_updated(
  peer: Peer,
  announce_data: AnnounceRequest,
  pool: web::Data<PgPool>,
  ip: Option<IpNetwork>,
  user_agent: Option<&str>,
) -> Result<HttpResponse, APIError> {
  let _peer =
    sqlx::query_as::<_, Peer>("SELECT * FROM peer_update($1, $2, $3, $4, $5, $6, $7, $8)")
      .bind(peer.id)
      .bind(ip)
      .bind(announce_data.port as i32)
      .bind(user_agent)
      .bind(announce_data.uploaded)
      .bind(announce_data.downloaded)
      .bind(announce_data.left)
      .bind(Behaviour::Lurker)
      .bind(Utc::now())
      .fetch_optional(pool.get_ref())
      .await?
      .ok_or_else(|| PeerError::NotUpdated)?;

  Ok(HttpResponse::Ok().finish())
}

async fn handle_peer_paused(
  _peer: Peer,
  _announce_data: AnnounceRequest,
) -> Result<HttpResponse, APIError> {
  Ok(HttpResponse::Ok().finish())
}
