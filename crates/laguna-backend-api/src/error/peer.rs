use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{http::header::ContentType, HttpResponse, ResponseError};
use laguna_backend_tracker::http::announce::AnnounceReply;
use laguna_backend_tracker::prelude::info_hash::InfoHash;
use laguna_backend_tracker::prelude::peer::PeerId;
use laguna_backend_tracker_common::announce::AnnounceEvent;
use laguna_backend_tracker_common::peer::PeerStream;

use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum PeerError<const N: usize> {
  NotFound(PeerId),
  UnknownTorrent(InfoHash<N>),
  UnexpectedEvent {
    event: AnnounceEvent,
    message: String,
  },
  NotCreated,
  NotUpdated,
  SqlxError(sqlx::Error),
}

impl<const N: usize> From<sqlx::Error> for PeerError<N> {
  fn from(value: sqlx::Error) -> Self {
    Self::SqlxError(value)
  }
}

impl<const N: usize> fmt::Display for PeerError<N> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::UnexpectedEvent { event, message } => f.write_fmt(format_args!(
        "Nepričakovan dogodek {:?}. {}",
        event, message
      )),
      Self::NotCreated => f.write_str("Peer ni bil ustvarjen."),
      Self::UnknownTorrent(info_hash) => f.write_fmt(format_args!(
        "Torrent z info_hash {} ne obstaja.",
        info_hash
      )),
      // NOTE: Don't output this.
      Self::SqlxError(_) => f.write_str("Napaka v PB."),
      Self::NotFound(peer_id) => {
        f.write_fmt(format_args!("Peer z peer_id {} ne obstaja.", peer_id))
      },
      Self::NotUpdated => f.write_str("Peer ni bil posodobljen."),
    }
  }
}

impl<const N: usize> ResponseError for PeerError<N> {
  fn status_code(&self) -> StatusCode {
    match self {
      Self::NotFound(_) => StatusCode::BAD_REQUEST,
      Self::UnknownTorrent(_) => StatusCode::BAD_REQUEST,
      Self::UnexpectedEvent { .. } => StatusCode::BAD_REQUEST,
      Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Self::NotCreated => StatusCode::BAD_REQUEST,
      Self::NotUpdated => StatusCode::BAD_REQUEST,
    }
  }

  /// Peer error responses have to be send back as bencoded responses with "failure reason" set.
  fn error_response(&self) -> HttpResponse<BoxBody> {
    HttpResponse::build(self.status_code())
      .content_type(ContentType::plaintext())
      .body(
        serde_bencode::to_bytes::<AnnounceReply>(&AnnounceReply {
          failure_reason: Some(self.to_string()),
          warning_message: None,
          incomplete: 0,
          complete: 0,
          interval: 0,
          min_interval: None,
          tracker_id: None,
          peers: PeerStream::Dict(vec![]),
        })
        .unwrap(),
      )
  }
}
