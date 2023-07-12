use actix_web::http::header::ContentType;
use actix_web::{body::BoxBody, http::StatusCode};
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use super::APIError;

#[derive(Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum TorrentError {
    DoesNotExist,
}

impl From<TorrentError> for APIError {
    fn from(torrent_error: TorrentError) -> Self {
        Self::TorrentError(torrent_error)
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
            Self::DoesNotExist => StatusCode::NOT_FOUND,
        }
    }
}
