use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum RatingError {
  AlreadyRated,
  NotDeleted,
  NotCreated,
}

impl fmt::Display for RatingError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::AlreadyRated => f.write_str("Torrent je že ocenjen."),
      Self::NotDeleted => f.write_str("Ocena ni bila uspešno izbrisana."),
      Self::NotCreated => f.write_str("Ocena ni bila uspešno ustvarjena."),
    }
  }
}

impl ResponseError for RatingError {
  fn status_code(&self) -> StatusCode {
    match self {
      Self::AlreadyRated => StatusCode::BAD_REQUEST,
      Self::NotDeleted => StatusCode::BAD_REQUEST,
      Self::NotCreated => StatusCode::BAD_REQUEST,
    }
  }

  fn error_response(&self) -> HttpResponse<BoxBody> {
    HttpResponse::build(self.status_code()).json(self.to_string())
  }
}
