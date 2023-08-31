use crate::error::peer::PeerError;

use actix_web::http::header::USER_AGENT;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use laguna_backend_dto::user::UserDTO;

use laguna_backend_model::behaviour::Behaviour;
use laguna_backend_model::peer::Peer;

use laguna_backend_model::torrent::Torrent;
use laguna_backend_tracker::http::announce::{Announce, AnnounceReply};

use laguna_backend_tracker_common::announce::AnnounceEvent;

use laguna_backend_tracker_common::peer::PeerStream;
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::PgPool;

use std::str::FromStr;

/// GET `/peer/announce`
pub async fn peer_announce<const N: usize>(
  req: HttpRequest,
  announce_data: web::Query<Announce<N>>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, PeerError<N>> {
  /*
  let user = sqlx::query_as::<_, User>("SELECT * FROM \"User\" INNER JOIN \"Peer\" ON \"User\".id = \"Peer\".user_id WHERE \"Peer\".peer_id = $1")
    .bind(announce_data.peer_id)
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserDTO::from)
    .ok_or(PeerError::NotCreated)?;
  */
  let _torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM torrent_get($1)")
    .bind(&announce_data.info_hash)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or(PeerError::UnknownTorrent(announce_data.info_hash.clone()))?;

  let _maybe_peer = sqlx::query_as::<_, Peer>("SELECT * FROM peer_get($1)")
    .bind(announce_data.peer_id)
    .fetch_optional(pool.get_ref())
    .await?;

  let _ip = announce_data.ip.map(IpNetwork::from).or_else(|| {
    req
      .connection_info()
      .realip_remote_addr() // go over proxy (if it exists)
      .and_then(|maybe_ip| IpNetwork::from_str(maybe_ip).ok())
  });

  let _user_agent = req
    .headers()
    .get(USER_AGENT)
    .map(|hv| hv.to_str().expect("Cannot convert header value to str"));

  Ok(
    HttpResponse::Ok().body(
      serde_bencode::to_bytes(&AnnounceReply {
        failure_reason: Some(String::from("Not implemented")),
        warning_message: None,
        interval: 10,
        min_interval: None,
        tracker_id: None,
        complete: 0,
        incomplete: 1,
        peers: PeerStream::Dict(vec![]),
      })
      .unwrap(),
    ),
  )
  /*
  handle_peer_request(
    maybe_peer,
    announce_data.into_inner(),
    pool,
    ip,
    user_agent,
    user,
  )
  .await*/
}

async fn handle_peer_request<const N: usize>(
  maybe_peer: Option<Peer>,
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
  ip: Option<IpNetwork>,
  user_agent: Option<&str>,
  user: UserDTO,
) -> Result<HttpResponse, PeerError<N>> {
  let event = announce_data.event.unwrap_or(AnnounceEvent::Empty);
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
      None => Err(PeerError::<N>::UnexpectedEvent {
        event: AnnounceEvent::Completed,
        message: String::from("Inexistant peer sent completion."),
      }),
    },
    AnnounceEvent::Stopped => match maybe_peer {
      Some(peer) => handle_peer_stopped(peer, announce_data).await,
      None => Err(PeerError::<N>::UnexpectedEvent {
        event: AnnounceEvent::Stopped,
        message: String::from("Inexistant peer sent stop."),
      }),
    },
    AnnounceEvent::Updated => match maybe_peer {
      Some(peer) => handle_peer_updated(peer, announce_data, pool, ip, user_agent).await,
      None => Err(PeerError::<N>::UnexpectedEvent {
        event: AnnounceEvent::Updated,
        message: String::from("Inexistant peer sent update."),
      }),
    },
    AnnounceEvent::Paused => match maybe_peer {
      Some(peer) => handle_peer_paused(peer, announce_data).await,
      None => Err(PeerError::<N>::UnexpectedEvent {
        event: AnnounceEvent::Paused,
        message: String::from("Inexistant peer sent pause."),
      }),
    },
    AnnounceEvent::Empty => Ok(HttpResponse::UnprocessableEntity().finish()),
  }
}

async fn handle_peer_started<const N: usize>(
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
  ip: Option<IpNetwork>,
  user_agent: Option<&str>,
  user: UserDTO,
) -> Result<HttpResponse, PeerError<N>> {
  let _peer =
    sqlx::query_as::<_, Peer>("SELECT * FROM peer_insert($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
      .bind(announce_data.peer_id)
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
      .ok_or(PeerError::<N>::NotCreated)?;
  Ok(HttpResponse::Ok().finish())
}

async fn handle_peer_stopped<const N: usize>(
  _peer: Peer,
  _announce_data: Announce<N>,
) -> Result<HttpResponse, PeerError<N>> {
  Ok(HttpResponse::Ok().finish())
}

async fn handle_peer_completed<const N: usize>(
  _peer: Peer,
  _announce_data: Announce<N>,
) -> Result<HttpResponse, PeerError<N>> {
  Ok(HttpResponse::Ok().finish())
}

async fn handle_peer_updated<const N: usize>(
  peer: Peer,
  announce_data: Announce<N>,
  pool: web::Data<PgPool>,
  ip: Option<IpNetwork>,
  user_agent: Option<&str>,
) -> Result<HttpResponse, PeerError<N>> {
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
      .ok_or(PeerError::<N>::NotUpdated)?;

  Ok(HttpResponse::Ok().finish())
}

async fn handle_peer_paused<const N: usize>(
  _peer: Peer,
  _announce_data: Announce<N>,
) -> Result<HttpResponse, PeerError<N>> {
  Ok(HttpResponse::Ok().finish())
}
