pub mod peer;
pub mod torrent;
pub mod user;

use actix_jwt_auth_middleware::AuthError;
use actix_multipart::MultipartError;
use actix_web::http::header::ContentType;
use actix_web::{body::BoxBody, http::StatusCode};
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use std::io;

#[derive(Debug, Display)]
pub enum APIError {
    SqlxError(sqlx::Error),
    IOError(io::Error),
    AuthError(AuthError),
    UserError(user::UserError),
    MultipartError(MultipartError),
    TorrentError(torrent::TorrentError),
    PeerError(peer::PeerError),
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
        Self::PeerError(peer_error.into())
    }
}

impl From<torrent::BencodeError> for APIError {
    fn from(bencode_error: torrent::BencodeError) -> Self {
        Self::TorrentError(bencode_error.into())
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

impl ResponseError for APIError {
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
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::TorrentError(torrent_error) => torrent_error.status_code(),
            Self::PeerError(peer_error) => peer_error.status_code(),
            Self::UserError(user_error) => user_error.status_code(),
            Self::AuthError(auth_error) => auth_error.status_code(),
            Self::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::MultipartError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
