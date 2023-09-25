use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{http::header::ContentType, HttpResponse, ResponseError};
use bendy::encoding::ToBencode;
use bendy::{decoding, encoding};
use laguna_backend_model::download::DownloadHash;
use laguna_backend_tracker::http::announce::AnnounceReply;
use laguna_backend_tracker::prelude::info_hash::InfoHash;
use laguna_backend_tracker::prelude::peer::PeerId;
use laguna_backend_tracker_common::announce::AnnounceEvent;
use laguna_backend_tracker_common::peer::PeerStream;
use uuid::Uuid;

use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum PeerError<const N: usize> {
  NotFound(PeerId),
  DownloadNotFound(DownloadHash),
  UnknownTorrent(InfoHash<N>),
  UnknownUser(Uuid),
  UnexpectedEvent {
    event: AnnounceEvent,
    message: String,
  },
  NotCreated,
  NotUpdated,
  SqlxError(sqlx::Error),
  BencodeDecodeError(decoding::Error),
  BencodeEncodeError(encoding::Error),
}

impl<const N: usize> From<sqlx::Error> for PeerError<N> {
  fn from(value: sqlx::Error) -> Self {
    Self::SqlxError(value)
  }
}

impl<const N: usize> From<decoding::Error> for PeerError<N> {
  fn from(value: decoding::Error) -> Self {
    Self::BencodeDecodeError(value)
  }
}

impl<const N: usize> From<encoding::Error> for PeerError<N> {
  fn from(value: encoding::Error) -> Self {
    Self::BencodeEncodeError(value)
  }
}

impl<const N: usize> fmt::Display for PeerError<N> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::UnexpectedEvent { event, message } => f.write_fmt(format_args!(
        "Nepričakovan dogodek {:?}. {}.",
        event, message
      )),
      Self::NotCreated => f.write_str("Peer ni bil ustvarjen."),
      Self::UnknownTorrent(info_hash) => f.write_fmt(format_args!(
        "Torrent z info_hash {} ne obstaja na strežniku. Za dodajanje torrenta uporabite `api/torrent/put`.",
        info_hash
      )),
      // NOTE: Don't output this.
      Self::SqlxError(_) => f.write_str("Napaka v PB."),
      Self::BencodeDecodeError(_) => f.write_str("Napaka pri dekodiranju bencode."),
      Self::BencodeEncodeError(_) => f.write_str("Napaka pri kodiranju bencode."),
      Self::NotFound(peer_id) => {
        f.write_fmt(format_args!("Peer z peer_id {} ne obstaja.", peer_id))
      },
      Self::DownloadNotFound(download_hash) => {
        f.write_fmt(format_args!("Torrent download hash {} ne obstaja.", download_hash))
      },
      Self::NotUpdated => f.write_str("Peer ni bil posodobljen."),
      Self::UnknownUser(id) => {
        f.write_fmt(format_args!("Uporabnik z id {} ne obstaja.", id))
      }
    }
  }
}

impl<const N: usize> ResponseError for PeerError<N> {
  fn status_code(&self) -> StatusCode {
    // We can send 200 status codes back.
    StatusCode::OK
  }

  /// Peer error responses have to be send back as bencoded responses with "failure reason" set.
  fn error_response(&self) -> HttpResponse<BoxBody> {
    HttpResponse::build(self.status_code())
      .content_type(ContentType::plaintext())
      .body(
        AnnounceReply {
          failure_reason: Some(self.to_string()),
          warning_message: None,
          incomplete: 0,
          complete: 0,
          interval: 0,
          min_interval: None,
          tracker_id: None,
          peers: PeerStream::Dict(vec![]),
        }
        .to_bencode()
        .unwrap(),
      )
  }
}
