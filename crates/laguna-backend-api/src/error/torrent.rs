use actix_web::http::header::ContentType;
use actix_web::{body::BoxBody, http::StatusCode};
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
pub use serde_bencode::Error as BencodeError;

#[derive(Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum TorrentError {
    DoesNotExist,
    BencodeError(String),
    DidntCreate,
}

impl From<BencodeError> for TorrentError {
    fn from(value: BencodeError) -> Self {
        Self::BencodeError(value.to_string())
    }
}

impl ResponseError for TorrentError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::plaintext())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::DoesNotExist => StatusCode::BAD_REQUEST,
            Self::BencodeError(_) => StatusCode::BAD_REQUEST,
            Self::DidntCreate => StatusCode::BAD_REQUEST,
        }
    }
}
