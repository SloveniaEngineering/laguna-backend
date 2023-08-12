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
/// # Example
/// ### Request
/// ```sh
/// curl -X GET \
///      -i 'http://127.0.0.1:6969/api/torrent/<20 bytes of info hash>'
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4'
/// ```
/// ### Response
/// #### Body
/// ```json
/// {
///   "id": "00f045ac-1f4d-4601-b2e3-87476dc462e6",
///   "announce_url": "http://127.0.0.1:6969/api/torrent/announce",
///   "length": 100,
///   "title": "TEST (2020)",
///   "file_name": "test2020.txt",
///   "nfo": null,
///   "leech_count": 0,
///   "seed_count": 0,
///   "completed_count": 0,
///   "speedlevel": "Lowspeed",
///   "info_hash": "aae8b4b6a0b9b6b5b4b6b5b4b6b5b4b6b5b4b6b5",
///   "uploaded_at": "2023-07-10T12:42:32.396647Z",
///   "uploaded_by": "00f045ac-1f4d-4601-b2e3-87476dc462e6",
///   "modded_at": null,
///   "modded_by": null
/// }
/// ```
/// #### Status Code
/// |Code|Description|
/// |---|---|
/// |200 OK|Returns [`TorrentDTO`]|
/// |400 Bad Request|Torrent not found|
/// |401 Unauthorized|Authentication/Authorization failed to process user|
pub async fn torrent_get(
  info_hash: web::Path<InfoHash>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let torrent = sqlx::query_as::<_, Torrent>("SELECT * FROM torrent_get($1)")
    .bind(info_hash.into_inner())
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| TorrentError::DoesNotExist)?;
  Ok(HttpResponse::Ok().json(TorrentDTO::from(torrent)))
}

/// `PATCH /api/torrent/`
/// # Example
/// ### Request
/// ```sh
/// curl -X PATCH \
///      -i 'http://127.0.0.1:6969/api/torrent/' \
///      -H 'Content-Type: application/json' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4' \
///      --data '{
///         "id": "<20 bytes of info_hash>",
///         "title": "TEST (2020)",
///         "file_name": "test_upload",
///         "nfo": null,
///         "modded_by": null
///      }'
/// ```
/// ### Response
/// #### Body
/// ```json
/// {
///   "id": "00f045ac-1f4d-4601-b2e3-87476dc462e6",
///   "announce_url": "http://127.0.0.1:6969/api/torrent/announce",
///   "length": 100,
///   "title": "Test, the movie (2020)",
///   "file_name": "test_upload.mp4",
///   "nfo": null,
///   "leech_count": 0,
///   "seed_count": 0,
///   "completed_count": 0,
///   "speedlevel": "Lowspeed",
///   "info_hash": "aae8b4b6a0b9b6b5b4b6b5b4b6b5b4b6b5b4b6b5",
///   "uploaded_at": "2023-07-10T12:42:32.396647Z",
///   "uploaded_by": "ffff45ac-1f4d-46f1-b2e3-87476dc462e6",
///   "modded_at": null,
///   "modded_by": null,
/// }
/// ```
/// #### Status Code
/// |Code|Description|
/// |---|---|
/// |200 OK|Successful patch. Returns updated [`TorrentDTO`]|
/// |400 Bad Request|Didnt patch. Invalid data|
/// |401 Unauthorized|Authentication/Authorization failed to process user|
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
/// # Example
/// ### Request
/// ```sh
/// curl -X PUT \
///      -i 'http://127.0.0.1:6969/api/torrent/' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODk3NjY1NjksImlhdCI6MTY4OTc2NjUwOSwiaWQiOiI5NGU3MWE3My1mNDkyLTQwZTYtOTM1YS1mN2RiNjFlMjI1MTciLCJ1c2VybmFtZSI6InRlc3R4eHgiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTlUMTE6MzQ6MzYuMDMyMjc5WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTE5VDExOjM1OjA5LjIzOTcyN1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.4PsBEXr3Zvnop2ztqt1rdnG1CXxIPnB-RYeGU74hrhw' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4' \
///      -H 'Content-Type: multipart/form-data'  \
///      -H 'Content-Disposition: form-data; filename=alice.torrent' \
///      -F 'upload=@crates/laguna-backend-api/fixtures/webtorrent-fixtures/fixtures/alice.torrent'
/// ```
/// ### Response
/// #### Status Code
/// |Status Code|Description|
/// |---|---|
/// |200 OK|Successful upload|
/// |208 Already Reported|Torrent with that `info_hash` already exists|
/// |400 Bad Request|Didnt create torrent due to invalid data|
/// |422 Unprocessable Entity|Invalid data format or corrupt multipart form-data|
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
  Ok(HttpResponse::UnprocessableEntity().finish())
}
