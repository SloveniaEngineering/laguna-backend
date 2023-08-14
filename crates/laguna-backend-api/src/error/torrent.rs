use actix_web::http::header::ContentType;
use actix_web::{body::BoxBody, http::StatusCode};
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TorrentError {
  DidntFind,
  DidntCreate,
  DidntUpdate,
  Invalid,
}

impl fmt::Display for TorrentError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::DidntFind => f.write_str("Torrent ne obstaja."),
      Self::DidntCreate => f.write_str("Torrent ni bil ustvarjen."),
      Self::DidntUpdate => f.write_str("Torrent ni bil posodobljen."),
      Self::Invalid => f.write_str("Torrent ni v pravilni obliki."),
    }
  }
}

impl ResponseError for TorrentError {
  fn status_code(&self) -> StatusCode {
    match self {
      Self::DidntFind => StatusCode::BAD_REQUEST,
      Self::DidntCreate => StatusCode::BAD_REQUEST,
      Self::DidntUpdate => StatusCode::BAD_REQUEST,
      Self::Invalid => StatusCode::UNPROCESSABLE_ENTITY,
    }
  }

  fn error_response(&self) -> HttpResponse<BoxBody> {
    HttpResponse::build(self.status_code())
      .content_type(ContentType::plaintext())
      .body(self.to_string())
  }
}
