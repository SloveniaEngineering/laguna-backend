use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use bendy::decoding::FromBencode;
use bendy::encoding::ToBencode;

use laguna_backend_tracker_common::info_hash::SHA1_LENGTH;
use sha1::Sha1;

use actix_multipart_extract::Multipart;
use chrono::{DateTime, Utc};
use laguna_backend_dto::torrent::{TorrentDTO, TorrentFile, TorrentPatchDTO, TorrentPutDTO};
use laguna_backend_dto::user::UserDTO;
use laguna_backend_middleware::mime::{APPLICATION_LAGUNA_JSON_VERSIONED, APPLICATION_XBITTORRENT};
use laguna_backend_model::behaviour::Behaviour;
use laguna_backend_model::genre::Genre;
use laguna_backend_model::peer::Peer;
use laguna_backend_model::torrent::Torrent;

use digest::Digest;
use laguna_backend_model::speedlevel::SpeedLevel;

use laguna_backend_model::download::{Download, DownloadHash};
use sqlx::PgPool;

use laguna_backend_tracker::prelude::info_hash::InfoHash;
use uuid::Uuid;

use crate::error::download::DownloadError;
use crate::error::{torrent::TorrentError, APIError};
use sha2::Sha256;

#[allow(missing_docs)]
#[utoipa::path(
  get,
  path = "/api/torrent/{info_hash}",
  responses(
    (status = 200, description = "Returns torrent.", body = Torrent, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    (status = 400, description = "Torrent not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
  )
)]
pub async fn torrent_get<const N: usize>(
  info_hash: web::Path<InfoHash<N>>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let torrent = sqlx::query_file_as!(
    Torrent,
    "queries/torrent_get.sql",
    info_hash.into_inner() as _
  )
  .fetch_optional(pool.get_ref())
  .await?
  .map(TorrentDTO::from)
  .ok_or(TorrentError::NotFound)?;
  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(torrent),
  )
}

#[allow(missing_docs)]
#[utoipa::path(
  get,
  path = "/api/torrent/{info_hash}/raw",
  responses(
    (status = 200, description = "Returns torrent.", body = Vec<u8>, content_type = "application/x-bittorrent"),
    (status = 400, description = "Torrent not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
  )
)]
pub async fn torrent_get_raw<const N: usize>(
  info_hash: web::Path<InfoHash<N>>,
  pool: web::Data<PgPool>,
  user: UserDTO,
  domestic_announce_url: web::Data<String>,
) -> Result<HttpResponse, APIError> {
  let info_hash = info_hash.into_inner();
  let download = sqlx::query_file_as!(
    Download::<N>,
    "queries/download_lookup.sql",
    info_hash as _,
    user.id
  )
  .fetch_optional(pool.get_ref())
  .await?;

  let torrent_bytes = sqlx::query_file_as!(Torrent, "queries/torrent_get.sql", info_hash as _)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or(TorrentError::NotFound)?
    .raw;

  let mut torrent = TorrentFile::from_bencode(&torrent_bytes)?;

  let down_ts = Utc::now();
  let down_hash = Sha256::digest(
    [
      &info_hash.0,
      &user.id.to_bytes_le()[..],
      &down_ts.timestamp().to_le_bytes(),
    ]
    .concat(),
  );

  // overwrite annouce url by adding down_hash
  torrent.announce_url = Some(format!(
    "{}?down_hash={}",
    domestic_announce_url.into_inner(),
    DownloadHash::from(down_hash.to_vec())
  ));

  match download {
    Some(_) => {
      sqlx::query_file_as!(
        Download::<N>,
        "queries/download_update.sql",
        down_ts,
        down_hash.as_slice(),
        info_hash as _,
        user.id
      )
      .fetch_optional(pool.get_ref())
      .await?
      .map(drop)
      .ok_or(DownloadError::NotUpdated)?;
    },
    None => {
      sqlx::query_file_as!(
        Download::<N>,
        "queries/download_insert.sql",
        info_hash as _,
        user.id,
        down_ts,
        down_hash.as_slice()
      )
      .fetch_optional(pool.get_ref())
      .await?
      .map(drop)
      .ok_or(DownloadError::NotCreated)?;
    },
  }

  let torrent_bytes = torrent.to_bencode()?;

  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_XBITTORRENT)
      .body(torrent_bytes),
  )
}

#[allow(missing_docs)]
#[utoipa::path(
  patch,
  path = "/api/torrent/{info_hash}",
  responses(
    (status = 200, description = "Returns updated torrent.", body = Torrent, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    (status = 400, description = "Torrent not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
  ),
  request_body = TorrentPatchDTO,
)]
pub async fn torrent_patch<const N: usize>(
  info_hash: web::Path<InfoHash<N>>,
  torrent_dto: Json<TorrentPatchDTO>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let torrent_patch = torrent_dto.into_inner();
  let torrent_dto = sqlx::query_file_as!(
    Torrent,
    "queries/torrent_update.sql",
    torrent_patch.nfo,
    torrent_patch.genre as _,
    info_hash.into_inner() as _
  )
  .fetch_optional(pool.get_ref())
  .await?
  .map(TorrentDTO::from)
  .ok_or(TorrentError::NotUpdated)?;
  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(torrent_dto),
  )
}

#[allow(missing_docs)]
#[utoipa::path(
  put,
  path = "/api/torrent",
  responses(
    (status = 200, description = "Returns created torrent.", body = Torrent, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    (status = 400, description = "Torrent not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
  ),
  request_body(content = TorrentPutDTO, content_type = "multipart/form-data"),
)]
pub async fn torrent_put<const N: usize>(
  form: Multipart<TorrentPutDTO>,
  pool: web::Data<PgPool>,
  domestic_announce_url: web::Data<String>,
  user: UserDTO,
) -> Result<HttpResponse, APIError> {
  /*if user.role < Role::Verified {
    return Err(UserError::InvalidCredentials.into());
  }*/
  // TODO: Bytes scanning middleware
  if form.torrent.content_type != APPLICATION_XBITTORRENT {
    return Ok(HttpResponse::UnsupportedMediaType().finish());
  }
  let torrent_file = TorrentFile::from_bencode(&form.torrent.bytes)?;
  // Deny torrents with announce list present
  if torrent_file.announce_list.is_some() {
    return Err(TorrentError::Invalid.into());
  }

  // Deny torrents with foreign announce url
  let domestic_announce_url = domestic_announce_url.into_inner();
  match torrent_file.announce_url {
    Some(announce_url_inner) if announce_url_inner != *domestic_announce_url => {
      eprintln!("{} ::: {}", announce_url_inner, *domestic_announce_url);
      return Err(TorrentError::Invalid.into());
    },
    // TODO: Remove `None` and adjust tests so that torrents with domestic announce url are used.
    Some(_) | None => (),
  }

  match torrent_file.info.private {
    Some(private) if private != 1 => return Err(TorrentError::Invalid.into()),
    // TODO: Remove `None` and adjust tests so that torrents with private flag are used.
    Some(_) | None => (),
  }

  let info_hash = Sha1::digest(torrent_file.info.to_bencode()?);
  // TODO: BitTorrent v2 needs SHA256_LENGTH
  let info_hash = InfoHash::<SHA1_LENGTH>(info_hash.try_into().unwrap());
  let maybe_torrent =
    sqlx::query_file_as!(Torrent, "queries/torrent_get.sql", info_hash.clone() as _)
      .fetch_optional(pool.get_ref())
      .await?;
  if maybe_torrent.is_some() {
    return Ok(HttpResponse::AlreadyReported().finish());
  }

  let torrent_dto = sqlx::query_file_as!(
    Torrent,
    "queries/torrent_insert.sql",
    info_hash as _,
    form.torrent.bytes, // torrent is already bencoded so we can just insert it
    torrent_file.announce_url,
    torrent_file.info.length,
    torrent_file.info.name.clone(),
    torrent_file.nfo,
    None::<Genre> as _,
    0,
    0,
    0,
    SpeedLevel::Lowspeed as _,
    false,
    torrent_file.creation_date,
    torrent_file.created_by,
    Utc::now(),
    user.id,
    None::<DateTime::<Utc>>,
    None::<Uuid>
  )
  .fetch_optional(pool.get_ref())
  .await?
  .map(TorrentDTO::from)
  .ok_or(TorrentError::NotCreated)?;

  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(torrent_dto),
  )
}

#[allow(missing_docs)]
#[utoipa::path(
  delete,
  path = "/api/torrent/{info_hash}",
  responses(
    (status = 200, description = "Returns deleted torrent.", body = Torrent, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    (status = 400, description = "Torrent not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
  )
)]
pub async fn torrent_delete<const N: usize>(
  info_hash: web::Path<InfoHash<N>>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let torrent_dto = sqlx::query_file_as!(
    Torrent,
    "queries/torrent_delete.sql",
    info_hash.into_inner() as _
  )
  .fetch_optional(pool.get_ref())
  .await?
  .map(TorrentDTO::from)
  .ok_or(TorrentError::NotFound)?;
  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(torrent_dto),
  )
}

#[allow(missing_docs)]
#[utoipa::path(
  get,
  path = "/api/torrent/{info_hash}/swarm",
  responses(
    (status = 200, description = "Returns torrent swarm.", body = Vec<Peer>, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    (status = 400, description = "Torrent not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
  )
)]
pub async fn torrent_swarm<const N: usize>(
  info_hash: web::Path<InfoHash<N>>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let swarm = sqlx::query_file_as!(
    Peer,
    "queries/torrent_swarm.sql",
    info_hash.into_inner() as _
  )
  .fetch_all(pool.get_ref())
  .await?;
  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(swarm),
  )
}
