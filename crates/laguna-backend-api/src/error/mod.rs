pub mod peer;
pub mod torrent;
pub mod user;

use actix_jwt_auth_middleware::AuthError;
use actix_multipart::MultipartError;
use actix_web::http::header::ContentType;
use actix_web::{body::BoxBody, http::StatusCode};
use actix_web::{error::ResponseError, HttpResponse};
use core::fmt;
use serde_bencode::Error as BencodeError;
use std::fmt::Formatter;
use std::io;

#[derive(Debug)]
pub enum APIError {
  SqlxError(sqlx::Error),
  IOError(io::Error),
  AuthError(AuthError),
  UserError(user::UserError),
  BencodeError(BencodeError),
  MultipartError(MultipartError),
  TorrentError(torrent::TorrentError),
  PeerError(peer::PeerError),
  HexError(hex::FromHexError),
}

impl From<io::Error> for APIError {
  fn from(value: io::Error) -> Self {
    Self::IOError(value)
  }
}

impl From<torrent::TorrentError> for APIError {
  fn from(torrent_error: torrent::TorrentError) -> Self {
    Self::TorrentError(torrent_error)
  }
}

impl From<user::UserError> for APIError {
  fn from(user_error: user::UserError) -> Self {
    Self::UserError(user_error)
  }
}

impl From<peer::PeerError> for APIError {
  fn from(peer_error: peer::PeerError) -> Self {
    Self::PeerError(peer_error)
  }
}
impl From<sqlx::Error> for APIError {
  fn from(value: sqlx::Error) -> Self {
    Self::SqlxError(value)
  }
}

impl From<AuthError> for APIError {
  fn from(value: AuthError) -> Self {
    Self::AuthError(value)
  }
}

impl From<MultipartError> for APIError {
  fn from(value: MultipartError) -> Self {
    Self::MultipartError(value)
  }
}

impl From<BencodeError> for APIError {
  fn from(value: BencodeError) -> Self {
    Self::BencodeError(value)
  }
}

impl From<hex::FromHexError> for APIError {
  fn from(value: hex::FromHexError) -> Self {
    Self::HexError(value)
  }
}

impl fmt::Display for APIError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::TorrentError(torrent_error) => f.write_fmt(format_args!("{}", torrent_error)),
      Self::PeerError(peer_error) => f.write_fmt(format_args!("{}", peer_error)),
      Self::UserError(user_error) => f.write_fmt(format_args!("{}", user_error)),
      Self::AuthError(auth_error) => f.write_fmt(format_args!("{}", auth_error)),
      Self::IOError(io_error) => f.write_fmt(format_args!("{}", io_error)),
      Self::SqlxError(sqlx_error) => f.write_fmt(format_args!("{}", sqlx_error)),
      Self::MultipartError(multipart_error) => f.write_fmt(format_args!("{}", multipart_error)),
      Self::BencodeError(bencode_error) => f.write_fmt(format_args!("{}", bencode_error)),
      Self::HexError(hex_error) => f.write_fmt(format_args!("{}", hex_error)),
    }
  }
}

impl ResponseError for APIError {
  fn status_code(&self) -> StatusCode {
    match self {
      Self::TorrentError(torrent_error) => torrent_error.status_code(),
      Self::PeerError(peer_error) => peer_error.status_code(),
      Self::UserError(user_error) => user_error.status_code(),
      Self::AuthError(auth_error) => auth_error.status_code(),
      Self::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Self::MultipartError(_) => StatusCode::UNPROCESSABLE_ENTITY,
      Self::BencodeError(_) => StatusCode::UNPROCESSABLE_ENTITY,
      Self::HexError(_) => StatusCode::UNPROCESSABLE_ENTITY,
    }
  }

  fn error_response(&self) -> HttpResponse<BoxBody> {
    match self {
      Self::TorrentError(torrent_error) => torrent_error.error_response(),
      Self::PeerError(peer_error) => peer_error.error_response(),
      Self::UserError(user_error) => user_error.error_response(),
      Self::AuthError(auth_error) => auth_error.error_response(),
      Self::IOError(io_error) => HttpResponse::build(self.status_code())
        .content_type(ContentType::plaintext())
        .body(io_error.to_string()),
      Self::SqlxError(sqlx_error) => HttpResponse::build(self.status_code())
        .content_type(ContentType::plaintext())
        .body(sqlx_error.to_string()),
      Self::MultipartError(multipart_error) => HttpResponse::build(self.status_code())
        .content_type(ContentType::plaintext())
        .body(multipart_error.to_string()),
      Self::BencodeError(bencode_error) => HttpResponse::build(self.status_code())
        .content_type(ContentType::plaintext())
        .body(bencode_error.to_string()),
      Self::HexError(hex_error) => {
        HttpResponse::build(self.status_code()).body(hex_error.to_string())
      },
    }
  }
}
