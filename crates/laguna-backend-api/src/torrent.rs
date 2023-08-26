use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use actix_web_validator::Json;

use chrono::Utc;
use laguna_backend_dto::torrent::{TorrentDTO, TorrentPatchDTO, TorrentPutDTO};
use laguna_backend_dto::user::UserDTO;
use laguna_backend_model::torrent::Torrent;

use digest::Digest;
use futures::{StreamExt, TryStreamExt};
use sha2::Sha256;
use sqlx::PgPool;

use laguna_backend_tracker::prelude::info_hash::{InfoHash, SHA256_LENGTH};

use crate::error::{torrent::TorrentError, APIError};

/// `GET /api/torrent/{info_hash}`
pub async fn torrent_get(
  info_hash: web::Path<InfoHash>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM torrent_get($1)")
    .bind(info_hash.into_inner())
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| TorrentError::DidntFind)?;
  Ok(HttpResponse::Ok().json(TorrentDTO::from(torrent)))
}

/// `PATCH /api/torrent/`
pub async fn torrent_patch(
  torrent_dto: Json<TorrentPatchDTO>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let torrent_dto = torrent_dto.into_inner();
  let torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM torrent_patch($1, $2, $3, $4)")
    .bind(torrent_dto.info_hash)
    .bind(torrent_dto.title)
    .bind(torrent_dto.file_name)
    .bind(torrent_dto.nfo.unwrap_or_default())
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| TorrentError::DidntUpdate)?;
  Ok(HttpResponse::Ok().json(TorrentDTO::from(torrent)))
}

/// `PUT /api/torrent/`
pub async fn torrent_put(
  mut payload: Multipart,
  pool: web::Data<PgPool>,
  user: UserDTO,
) -> Result<HttpResponse, APIError> {
  if let Some(mut field) = payload.try_next().await? {
    let content_type = field.content_type();
    if let None = content_type {
      return Ok(HttpResponse::BadRequest().finish());
    }
    let mut torrent_buf = Vec::new();
    while let Some(chunk) = field.next().await {
      torrent_buf.extend_from_slice(&chunk?);
    }
    let torrent_put_dto = serde_bencode::from_bytes::<TorrentPutDTO>(&torrent_buf)?;
    let info_hash = Sha256::digest(serde_bencode::to_bytes(&torrent_put_dto.info)?);
    let info_hash = InfoHash::from(<[u8; SHA256_LENGTH]>::try_from(info_hash.as_slice()).unwrap());
    let maybe_torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM torrent_get($1)")
      .bind(info_hash.clone())
      .fetch_optional(pool.get_ref())
      .await?;
    if let Some(_) = maybe_torrent {
      return Ok(HttpResponse::AlreadyReported().finish());
    }
    let _torrent = sqlx::query_as::<_, Torrent>(
      "SELECT * FROM torrent_insert($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(info_hash)
    .bind(torrent_put_dto.announce_url.unwrap_or_default())
    .bind(torrent_put_dto.title)
    .bind(torrent_put_dto.info.length)
    .bind(torrent_put_dto.info.name)
    .bind(torrent_put_dto.nfo)
    .bind(Utc::now())
    .bind(user.id)
    .bind(torrent_put_dto.speedlevel)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| TorrentError::DidntCreate)?;

    return Ok(HttpResponse::Ok().finish());
  }
  Err(TorrentError::Invalid.into())
}
