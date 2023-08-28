use actix_web::web::Bytes;
use actix_web::{web, HttpResponse};
use actix_web_validator::Json;

use chrono::Utc;
use laguna_backend_dto::torrent::{TorrentDTO, TorrentPatchDTO, TorrentPutDTO};
use laguna_backend_dto::user::UserDTO;
use laguna_backend_model::peer::Peer;
use laguna_backend_model::torrent::Torrent;

use digest::Digest;
use laguna_backend_tracker_common::info_hash::SHA1_LENGTH;
use sha1::Sha1;
use sqlx::PgPool;

use laguna_backend_tracker::prelude::info_hash::InfoHash;

use crate::error::{torrent::TorrentError, APIError};

/// `GET /api/torrent/{info_hash}`
pub async fn torrent_get(
  info_hash: web::Path<String>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let mut info_hash_raw = [0u8; SHA1_LENGTH];
  hex::decode_to_slice(info_hash.into_inner(), &mut info_hash_raw).map_err(APIError::from)?;
  let info_hash = InfoHash::from(info_hash_raw);
  let torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM torrent_get($1)")
    .bind(info_hash)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or(TorrentError::NotFound)?;
  Ok(HttpResponse::Ok().json(torrent))
}

/// `PATCH /api/torrent/`
pub async fn torrent_patch(
  info_hash: web::Path<String>,
  torrent_dto: Json<TorrentPatchDTO>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let mut info_hash_raw = [0u8; SHA1_LENGTH];
  hex::decode_to_slice(info_hash.into_inner(), &mut info_hash_raw).map_err(APIError::from)?;
  let info_hash = InfoHash::from(info_hash_raw);
  let torrent_patch = torrent_dto.into_inner();
  let torrent_dto = sqlx::query_as::<_, Torrent>("SELECT * FROM torrent_patch($1, $2, $3)")
    .bind(info_hash)
    .bind(torrent_patch.title)
    .bind(torrent_patch.nfo.unwrap_or_default())
    .fetch_optional(pool.get_ref())
    .await?
    .map(TorrentDTO::from)
    .ok_or(TorrentError::NotUpdated)?;
  Ok(HttpResponse::Ok().json(torrent_dto))
}

/// `PUT /api/torrent/`
pub async fn torrent_put(
  body: Bytes,
  pool: web::Data<PgPool>,
  user: UserDTO,
) -> Result<HttpResponse, APIError> {
  // TODO: Bytes scanning middleware
  let torrent_put_dto = serde_bencode::from_bytes::<TorrentPutDTO>(&body)?;
  // TODO: Support bittorrent v2 with sha256 (40 bytes aka 80 in repr)
  let info_hash = Sha1::digest(serde_bencode::to_bytes(&torrent_put_dto.info)?);
  let info_hash = InfoHash::from(<[u8; SHA1_LENGTH]>::try_from(info_hash.as_slice()).unwrap());
  let maybe_torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM torrent_get($1)")
    .bind(info_hash.clone())
    .fetch_optional(pool.get_ref())
    .await?;
  if maybe_torrent.is_some() {
    return Ok(HttpResponse::AlreadyReported().finish());
  }
  let torrent_dto = sqlx::query_as::<_, Torrent>(
    "SELECT * FROM torrent_insert($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
  )
  .bind(info_hash)
  .bind(Vec::<u8>::from(body))
  .bind(torrent_put_dto.announce_url.unwrap_or_default())
  .bind(torrent_put_dto.info.name.clone()) //  TODO: replace with title
  .bind(torrent_put_dto.info.length)
  .bind(torrent_put_dto.info.name)
  .bind(torrent_put_dto.nfo.unwrap_or_default())
  .bind(torrent_put_dto.creation_date)
  .bind(Utc::now())
  .bind(user.id)
  .fetch_optional(pool.get_ref())
  .await?
  .map(TorrentDTO::from)
  .ok_or(TorrentError::NotCreated)?;

  Ok(HttpResponse::Ok().json(torrent_dto))
}

/// `DELETE /api/torrent/{info_hash}`
pub async fn torrent_delete(
  info_hash: web::Path<String>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let mut info_hash_raw = [0u8; SHA1_LENGTH];
  hex::decode_to_slice(info_hash.into_inner(), &mut info_hash_raw).map_err(APIError::from)?;
  let info_hash = InfoHash::from(info_hash_raw);
  let torrent_dto = sqlx::query_as::<_, Torrent>("SELECT * FROM torrent_delete($1)")
    .bind(info_hash)
    .fetch_optional(pool.get_ref())
    .await?
    .map(TorrentDTO::from)
    .ok_or(TorrentError::NotFound)?;
  Ok(HttpResponse::Ok().json(torrent_dto))
}

/// `GET /api/torrent/{info_hash}/swarm`
pub async fn torrent_swarm(
  info_hash: web::Path<String>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let mut info_hash_raw = [0u8; SHA1_LENGTH];
  hex::decode_to_slice(info_hash.into_inner(), &mut info_hash_raw).map_err(APIError::from)?;
  let info_hash = InfoHash::from(info_hash_raw);
  let swarm = sqlx::query_as::<_, Peer>("SELECT * FROM torrent_swarm($1)")
    .bind(info_hash)
    .fetch_all(pool.get_ref())
    .await?;
  Ok(HttpResponse::Ok().json(swarm))
}
