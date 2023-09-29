use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub enum DownloadError {
  NotUpdated,
  NotCreated,
}

impl ResponseError for DownloadError {
  fn status_code(&self) -> StatusCode {
    match self {
      Self::NotUpdated => StatusCode::BAD_REQUEST,
      Self::NotCreated => StatusCode::BAD_REQUEST,
    }
  }

  fn error_response(&self) -> HttpResponse<BoxBody> {
    HttpResponse::build(self.status_code()).body(self.to_string())
  }
}

impl std::fmt::Display for DownloadError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::NotUpdated => f.write_str("Download ni bil posodobljen."),
      Self::NotCreated => f.write_str("Download ni bil ustvarjen."),
    }
  }
}
