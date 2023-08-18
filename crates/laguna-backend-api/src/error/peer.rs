use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{http::header::ContentType, HttpResponse, ResponseError};
use laguna_backend_tracker::prelude::info_hash::{InfoHash, SHA1_LENGTH};
use laguna_backend_tracker::prelude::peer::PeerId;
use laguna_backend_tracker_common::announce::AnnounceEvent;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerError {
  NotFound(PeerId),
  UnknownTorrent(InfoHash<SHA1_LENGTH>),
  UnexpectedEvent {
    event: AnnounceEvent,
    message: String,
  },
  NotCreated,
  NotUpdated,
}

impl fmt::Display for PeerError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::UnexpectedEvent { event, message } => f.write_fmt(format_args!(
        "Unexpected event {:?} received from client. {}",
        event, message
      )),
      Self::NotCreated => f.write_str("Failed to create peer."),
      Self::UnknownTorrent(info_hash) => f.write_fmt(format_args!(
        "No torrent with info_hash {} found.",
        info_hash
      )),
      Self::NotFound(peer_id) => {
        f.write_fmt(format_args!("No client with id {} was found.", peer_id))
      },
      Self::NotUpdated => f.write_str("Peer ni bil posodobljen."),
    }
  }
}

impl ResponseError for PeerError {
  fn status_code(&self) -> StatusCode {
    match self {
      Self::NotFound(_) => StatusCode::BAD_REQUEST,
      Self::UnknownTorrent(_) => StatusCode::BAD_REQUEST,
      Self::UnexpectedEvent { .. } => StatusCode::BAD_REQUEST,
      Self::NotCreated => StatusCode::BAD_REQUEST,
      Self::NotUpdated => StatusCode::BAD_REQUEST,
    }
  }

  fn error_response(&self) -> HttpResponse<BoxBody> {
    HttpResponse::build(self.status_code())
      .content_type(ContentType::plaintext())
      .body(self.to_string())
  }
}
