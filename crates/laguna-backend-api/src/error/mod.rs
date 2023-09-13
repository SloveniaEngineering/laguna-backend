pub mod peer;
pub mod rating;
pub mod torrent;
pub mod user;

use actix_jwt_auth_middleware::AuthError;

use actix_web::http::header::ContentType;
use actix_web::{body::BoxBody, http::StatusCode};
use actix_web::{error::ResponseError, HttpResponse};
use core::fmt;

use std::fmt::Formatter;
use std::io;

use self::rating::RatingError;

#[derive(Debug)]
pub enum APIError {
  SqlxError(sqlx::Error),
  IOError(io::Error),
  AuthError(AuthError),
  UserError(user::UserError),
  BencodeError(serde_bencode::Error),
  TorrentError(torrent::TorrentError),
  RatingError(rating::RatingError),
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

impl From<serde_bencode::Error> for APIError {
  fn from(value: serde_bencode::Error) -> Self {
    Self::BencodeError(value)
  }
}

impl From<RatingError> for APIError {
  fn from(value: RatingError) -> Self {
    Self::RatingError(value)
  }
}

impl fmt::Display for APIError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::TorrentError(torrent_error) => f.write_fmt(format_args!("{}", torrent_error)),
      Self::UserError(user_error) => f.write_fmt(format_args!("{}", user_error)),
      Self::AuthError(auth_error) => f.write_fmt(format_args!("{}", auth_error)),
      Self::IOError(io_error) => f.write_fmt(format_args!("{}", io_error)),
      Self::SqlxError(sqlx_error) => f.write_fmt(format_args!("{}", sqlx_error)),
      Self::BencodeError(bencode_error) => f.write_fmt(format_args!("{}", bencode_error)),
      Self::RatingError(rating_error) => f.write_fmt(format_args!("{}", rating_error)),
    }
  }
}

impl ResponseError for APIError {
  fn status_code(&self) -> StatusCode {
    match self {
      Self::TorrentError(torrent_error) => torrent_error.status_code(),
      Self::UserError(user_error) => user_error.status_code(),
      Self::AuthError(auth_error) => auth_error.status_code(),
      Self::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Self::BencodeError(_) => StatusCode::UNPROCESSABLE_ENTITY,
      Self::RatingError(rating_error) => rating_error.status_code(),
    }
  }

  fn error_response(&self) -> HttpResponse<BoxBody> {
    match self {
      Self::TorrentError(torrent_error) => torrent_error.error_response(),
      Self::UserError(user_error) => user_error.error_response(),
      Self::AuthError(auth_error) => auth_error.error_response(),
      Self::IOError(io_error) => HttpResponse::build(self.status_code())
        .content_type(ContentType::plaintext())
        .body(io_error.to_string()),
      Self::SqlxError(sqlx_error) => HttpResponse::build(self.status_code())
        .content_type(ContentType::plaintext())
        .body(sqlx_error.to_string()),
      Self::BencodeError(bencode_error) => HttpResponse::build(self.status_code())
        .content_type(ContentType::plaintext())
        .body(bencode_error.to_string()),
      Self::RatingError(rating_error) => rating_error.error_response(),
    }
  }
}
