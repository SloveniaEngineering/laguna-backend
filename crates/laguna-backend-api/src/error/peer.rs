use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{http::header::ContentType, HttpResponse, ResponseError};
use laguna_backend_tracker::prelude::info_hash::InfoHash;
use laguna_backend_tracker::prelude::peer::PeerId;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerError {
    DoesNotExist(PeerId),
    RequestedNonexistentTorrent(InfoHash),
    DidntCreate,
}

impl fmt::Display for PeerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DidntCreate => f.write_str("Failed to create peer."),
            Self::RequestedNonexistentTorrent(info_hash) => f.write_fmt(format_args!(
                "No torrent with info_hash {} can be requested.",
                info_hash
            )),
            Self::DoesNotExist(peer_id) => {
                f.write_fmt(format_args!("No client with id {} was found.", peer_id))
            }
        }
    }
}

impl ResponseError for PeerError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::DoesNotExist(_) => StatusCode::BAD_REQUEST,
            Self::RequestedNonexistentTorrent(_) => StatusCode::BAD_REQUEST,
            Self::DidntCreate => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::plaintext())
            .body(self.to_string())
    }
}
