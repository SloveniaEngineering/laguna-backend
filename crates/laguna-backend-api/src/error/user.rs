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
  DidntFind,
  ExclusiveAccess,
  DidntCreate,
  DidntUpdate,
}

impl fmt::Display for UserError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::InvalidCredentials => f.write_str("Uporabniško ime, elektronski naslov ali geslo napačno."),
      Self::ExclusiveAccess => f.write_str("Samo za ene oči."),
      Self::DidntFind => f.write_str("Zahtevan uporabnik ne obstaja."),
      Self::DidntCreate => f.write_str("Uporabnik ni bil ustvarjen."),
      Self::DidntUpdate => f.write_str("Uporabnik ni bil posodobljen."),
    }
  }
}

impl ResponseError for UserError {
  fn status_code(&self) -> StatusCode {
    match self {
      Self::ExclusiveAccess => StatusCode::FORBIDDEN,
      Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
      Self::DidntFind => StatusCode::BAD_REQUEST,
      Self::DidntCreate => StatusCode::BAD_REQUEST,
      Self::DidntUpdate => StatusCode::BAD_REQUEST,
    }
  }

  fn error_response(&self) -> HttpResponse<BoxBody> {
    HttpResponse::build(self.status_code())
      .content_type(ContentType::plaintext())
      .body(self.to_string())
  }
}
