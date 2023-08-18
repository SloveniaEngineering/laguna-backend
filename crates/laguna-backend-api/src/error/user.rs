use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use actix_web::{http::header::ContentType, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserError {
  InvalidCredentials,
  NotFound,
  Exclusive,
  NotCreated,
  NotUpdated,
}

impl fmt::Display for UserError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::InvalidCredentials => {
        f.write_str("Uporabniško ime, elektronski naslov ali geslo napačno.")
      },
      Self::Exclusive => f.write_str("Samo za ene oči."),
      Self::NotFound => f.write_str("Zahtevan uporabnik ne obstaja."),
      Self::NotCreated => f.write_str("Uporabnik ni bil ustvarjen."),
      Self::NotUpdated => f.write_str("Uporabnik ni bil posodobljen."),
    }
  }
}

impl ResponseError for UserError {
  fn status_code(&self) -> StatusCode {
    match self {
      Self::Exclusive => StatusCode::FORBIDDEN,
      Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
      Self::NotFound => StatusCode::BAD_REQUEST,
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
