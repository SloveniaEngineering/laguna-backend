use actix_web::{web, HttpResponse};

use laguna_backend_dto::torrent::TorrentDTO;
use laguna_backend_model::torrent::Torrent;
use laguna_backend_model::user::UserSafe;
use laguna_backend_model::{peer::Peer, user::User};

use laguna_backend_dto::user::UserDTO;
use laguna_backend_dto::{peer::PeerDTO, user::UserPatchDTO};

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{user::UserError, APIError};

/// `GET /api/user/me`
/// # Example
/// ### Request
/// ```sh
/// curl -X GET \
///      -H 'Content-Type: application/json' \
///      -i 'http://127.0.0.1:6969/api/user/me' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4'
/// ```
/// ### Response
/// #### Body
/// ```json
/// {
///   "id": "00f045ac-1f4d-4601-b2e3-87476dc462e6",
///   "username": "test",
///   "first_login": "2023-07-10T12:42:32.396647Z",
///   "last_login": "2023-07-10T12:43:19.216047Z",
///   "avatar_url": null,
///   "role": "Normie",
///   "behaviour": "Lurker",
///   "is_active": true,
///   "has_verified_email": false,
///   "is_history_private": true,
///   "is_profile_private": true
/// }
/// ```
/// #### Status Code
/// |Code|Description|
/// |--------|-----------|
/// |200 OK  |User was found. Returns [`UserDTO`]|
/// |401 Unauthorized|Authentication/Authorization middleware failed to authenticate user|
pub async fn user_me_get(user: UserDTO) -> Result<HttpResponse, APIError> {
  Ok(HttpResponse::Ok().json(user))
}

/// `GET /api/user/{id}`
/// # Example
/// ### Request
/// ```sh
/// curl -X GET \
///      -H 'Content-Type: application/json' \
///      -i 'http://127.0.0.1:6969/api/user/id/00f045ac-1f4d-4601-b2e3-87476dc462e6' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4'
/// ```
/// ### Response
/// #### Body
/// ```json
/// {
///     "id": "00f045ac-1f4d-4601-b2e3-87476dc462e6",
///     "username": "test",
///     "first_login": "2023-07-10T12:42:32.396647Z",
///     "last_login": "2023-07-10T12:43:19.216047Z",
///     "avatar_url": null,
///     "role": "Normie",
///     "behaviour": "Lurker",
///     "is_active": true,
///     "has_verified_email": false,
///     "is_history_private": true,
///     "is_profile_private": true
/// }
/// ```
/// #### Status Code
/// |Code|Description|
/// |--------|-----------|
/// |200 OK|User was found. Returns [`UserDTO`]|
/// |400 Bad Request|User was not found|
/// |401 Unauthorized|Authentication/Authorization middleware failed for user requesting this action|
pub async fn user_get(
  id: web::Path<Uuid>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let user = sqlx::query_as::<_, User>(r#"SELECT * FROM user_get($1)"#)
    .bind(id.into_inner())
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserSafe::from)
    .map(UserDTO::from)
    .ok_or_else(|| UserError::NotFound)?;
  Ok(HttpResponse::Ok().json(user))
}

/// `DELETE /api/user/me`
/// # Example
/// ### Request
/// ```sh
/// curl -X DELETE \
///      -i 'http://127.0.0.1:6969/api/user/me' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4'
/// ```
/// ### Response
/// #### Status Code
/// |Code|Description|
/// |--------|-----------|
/// |200 OK|User was deleted|
/// |400 Bad Request|User was not found|
/// |401 Unauthorized|Authentication/Authorization middleware failed for user requesting this action|
pub async fn user_me_delete(
  user: UserDTO,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  sqlx::query_as::<_, User>("SELECT * FROM user_delete($1)")
    .bind(user.id)
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserSafe::from)
    .map(drop) // Zero-ize immediately
    .ok_or_else(|| UserError::NotFound)?;
  Ok(HttpResponse::Ok().finish())
}

/// `PATCH /api/user/{id}`
/// # Example
/// ### Request
/// ```sh
/// curl -X PATCH \
///     -i 'http://127.0.0.1:6969/api/user/00f045ac-1f4d-4601-b2e3-87476dc462e6' \
///     -H 'Content-Type: application/json' \
///     -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///     -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4' \
///     --data '{
///              "avatar_url": null,
///              "is_history_private": true,
///              "is_profile_private": true,
///            }'
/// ```
/// ### Response
/// #### Body
/// ```json
/// {
///     "id": "00f045ac-1f4d-4601-b2e3-87476dc462e6",
///     "username": "test",
///     "first_login": "2023-07-10T12:42:32.396647Z",
///     "last_login": "2023-07-10T12:43:19.216047Z",
///     "avatar_url": null,
///     "role": "Normie",
///     "behaviour": "Lurker",
///     "is_active": true,
///     "has_verified_email": false,
///     "is_history_private": true,
///     "is_profile_private": true
/// }
/// ```
/// #### Status Code
/// |Code|Description|
/// |--------|-----------|
/// |200 OK|User was updated. Returns updated [`UserDTO`]|
/// |400 Bad Request|User was not found but permissions are sufficient|
/// |401 Unauthorized|Authentication/Authorization middleware failed for user requesting this action|
/// |403 Forbidden|User requesting this action is trying to patch a different user (not themself)|
pub async fn user_patch(
  user_id: web::Path<Uuid>,
  user_patch_dto: web::Json<UserPatchDTO>,
  current_user: UserDTO,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  // TODO: Should this be middleware?
  let user_id = user_id.into_inner();
  if user_id != current_user.id {
    return Err(UserError::Exclusive.into());
  }
  let user_patch_dto = user_patch_dto.into_inner();
  let user = sqlx::query_as::<_, User>("SELECT * FROM user_patch($1, $2, $3, $4)")
    .bind(user_id)
    .bind(user_patch_dto.is_history_private)
    .bind(user_patch_dto.is_profile_private)
    .bind(user_patch_dto.avatar_url.unwrap_or_default())
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserSafe::from)
    .map(UserDTO::from)
    .ok_or_else(|| UserError::NotUpdated)?;
  Ok(HttpResponse::Ok().json(user))
}

/// `GET /api/user/{id}/peers`
/// # Example
/// ### Request
/// ```sh
/// curl -X GET \
///     -i 'http://127.0.0.1:6969/api/user/00f045ac-1f4d-4601-b2e3-87476dc462e6/peers' \
///     -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///     -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4'
/// ```
/// ### Response
/// #### Body
/// ```json
/// [
///   {
///     "id": "<20 bytes of peer_id>",
///     "md5_hash": "aae0bfbf0b0b0b0b0b0b0b0b0b0b0b0b", // md5
///     "info_hash": "afaf9284efc8fae8f8a8f8a8f8a8f8a8", // sha-256 (40 bytes)
///     "ip": "127.0.0.1",
///     "port": "45701",
///     "agent": "curl/7.75.0",
///     "uploaded_bytes": 0,
///     "downloaded_bytes": 0,
///     "left_bytes": 0,
///     "behaviour": "Lurker",
///     "created_at": "2023-07-10T12:42:32.396647Z",
///     "updated_at": "2023-07-10T12:43:19.216047Z",
///     "user_id": "00f045ac-1f4d-4601-b2e3-87476dc462e6",
///   },
///   {
///     ...
///   }
/// ]
/// ```
/// #### Status Code
/// |Code|Description|
/// |--------|-----------|
/// |200 OK|[`Vec<PeerDTO>`] were found, if none were found returns empty [`Vec`]|
/// |401 Unauthorized|Authentication/Authorization middleware failed to authenticate user|
pub async fn user_peers_get(
  id: web::Path<Uuid>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let peers = sqlx::query_as::<_, Peer>("SELECT * FROM user_peers_get($1)")
    .bind(id.into_inner())
    .fetch_all(pool.get_ref())
    .await?
    .into_iter()
    .map(PeerDTO::from)
    .collect::<Vec<PeerDTO>>();
  Ok(HttpResponse::Ok().json(peers))
}

/// `GET /api/user/{id}/torrents`
/// # Example
/// ### Request
/// ```sh
/// curl -X GET \
///    -i 'http://127.0.0.1:6969/api/user/00f045ac-1f4d-4601-b2e3-87476dc462e6/torrents' \
///    -H 'X-Access-Token: ...' \
///   -H 'X-Refresh-Token: ...'
/// ```
/// ### Response
/// #### Body
/// ```json
/// [
///   {
///    "info_hash": [210,71,78,134,201,91,25,184,188,253,185,43,193,44,157,68,102,124,250,54],
///    "raw":[100,49,48,58,99,114,101,97,116,101,100,32,98,121,49,51,58,117,84,111,114,114,101,110,116,47,51,51,48,48,49,51,58,99,114,101,97,116,105,111,110,32,100,97,116,101,105,49,51,55,53,51,54,51,54,54,54,101,56,58,101,110,99,111,100,105,110,103,53,58,85,84,70,45,56,52,58,105,110,102,111,100,54,58,108,101,110,103,116,104,105,51,54,50,48,49,55,101,52,58,110,97,109,101,51,54,58,76,101,97,118,101,115,32,111,102,32,71,114,97,115,115,32,98,121,32,87,97,108,116,32,87,104,105,116,109,97,110,46,101,112,117,98,49,50,58,112,105,101,99,101,32,108,101,110,103,116,104,105,49,54,51,56,52,101,54,58,112,105,101,99,101,115,52,54,48,58,31,156,63,89,190,236,7,151,21,236,83,50,75,222,133,105,228,160,180,235,236,66,48,125,76,229,85,123,93,57,100,197,239,85,211,84,207,74,110,204,123,241,188,175,121,209,31,165,224,190,6,89,60,143,170,252,12,43,162,207,118,215,28,91,1,82,107,35,0,127,158,153,41,190,175,197,21,30,101,17,9,49,161,180,76,33,191,30,104,185,19,143,144,73,94,105,13,188,85,245,114,228,194,148,76,186,207,38,230,179,174,138,114,41,216,138,175,160,95,97,234,174,106,191,63,7,203,109,185,103,124,198,173,237,77,211,152,94,69,134,39,86,127,167,99,159,6,95,113,177,137,84,48,74,202,99,102,114,158,11,71,115,215,122,232,12,170,150,165,36,128,77,254,75,155,211,222,174,249,153,201,221,81,2,116,103,81,157,94,178,86,26,226,204,1,70,125,229,246,67,10,96,188,186,36,121,118,146,239,168,119,13,35,223,10,131,13,145,203,53,179,64,122,136,186,160,89,13,200,201,170,106,18,15,39,67,103,220,216,103,232,142,131,56,197,114,160,110,60,128,27,41,245,25,223,83,43,62,118,246,112,207,106,238,83,16,127,61,57,55,132,131,246,156,248,15,165,104,177,234,197,59,80,97,89,233,136,216,188,22,146,45,18,93,119,216,3,214,82,195,202,48,112,193,110,237,145,114,171,80,109,32,229,34,234,63,26,182,116,179,249,35,215,111,232,244,79,243,46,55,44,59,55,101,100,198,251,95,13,190,82,22,79,3,98,159,209,50,38,54,186,187,44,1,75,125,174,88,45,164,19,99,150,82,97,230,206,18,180,55,1,240,168,201,237,21,32,167,14,186,0,68,0,162,103,118,95,109,61,213,199,190,181,189,60,117,243,223,42,84,86,10,97,128,17,71,250,78,199,207,86,142,112,58,203,4,229,97,10,77,86,220,194,66,208,50,147,233,68,108,245,228,87,216,235,61,149,136,253,144,198,152,222,155,13,173,146,152,9,6,192,38,216,193,64,143,160,143,228,236,101,101],
///    "announce_url":null,"length":362017,
///    "title":"Leaves of Grass by Walt Whitman.epub",
///    "file_name":"Leaves of Grass by Walt Whitman.epub",
///    "nfo":null,
///    "leech_count":0,
///    "seed_count":0,
///    "completed_count":0,
///    "speedlevel":"Lowspeed",
///    "creation_date":"2013-08-01T13:27:46Z",
///    "uploaded_at":"2023-08-16T08:59:48.044603Z",
///    "uploaded_by":"00f045ac-1f4d-4601-b2e3-87476dc462e6",
///    "modded_at":null,
///    "modded_by":null
///   },
///   {
///    ...
///   }
/// ]
/// ```
/// #### Status Code
/// |Code|Description|
/// |--------|-----------|
/// |200 OK|[`Vec<TorrentDTO>`] were found, if none were found returns empty [`Vec`]|
/// |401 Unauthorized|Authentication/Authorization middleware failed to authenticate user|
pub async fn user_torrents_get(
  id: web::Path<Uuid>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let torrents = sqlx::query_as::<_, Torrent>("SELECT * FROM user_torrents_get($1)")
    .bind(id.into_inner())
    .fetch_all(pool.get_ref())
    .await?
    .into_iter()
    .map(TorrentDTO::from)
    .collect::<Vec<TorrentDTO>>();

  Ok(HttpResponse::Ok().json(torrents))
}
