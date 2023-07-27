use actix_web::{delete, get, web, HttpResponse};
use laguna_backend_model::{
    peer::{Peer, PeerDTO},
    user::{User, UserDTO},
};

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{user::UserError, APIError};

/// `GET /api/user/me`
/// # Example
/// ## Request
/// ```sh
/// curl -X GET \
///      -H 'Content-Type: application/json' \
///      -i 'http://127.0.0.1:6969/api/user/me' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4'
/// ```
/// ## Response
/// * Normal behaviour: HTTP/1.1 200 OK
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
/// * Otherwise its already filtered out by auth middleware.
#[get("/me")]
pub async fn get_me(user: UserDTO) -> Result<HttpResponse, APIError> {
    Ok(HttpResponse::Ok().json(user))
}

/// `GET /api/user/{id}`
/// # Example
/// ## Request
/// ```sh
/// curl -X GET \
///      -H 'Content-Type: application/json' \
///      -i 'http://127.0.0.1:6969/api/user/id/00f045ac-1f4d-4601-b2e3-87476dc462e6' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4'
/// ```
/// ## Response
/// * If user was found: HTTP/1.1 200 OK
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
/// * If user was not found: HTTP/1.1 404 Not Found
#[get("/{id}")]
pub async fn get_user(
    id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id,
                     username,
                     email,
                     password,
                     first_login,
                     last_login,
                     avatar_url,
                     role AS "role: _",
                     behaviour AS "behaviour: _",
                     is_active,
                     has_verified_email,
                     is_history_private,
                     is_profile_private
            FROM "User" WHERE id = $1"#,
        id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| UserError::DoesNotExist)?;
    Ok(HttpResponse::Ok().json(UserDTO::from(user)))
}

/// `DELETE /api/user/delete/me`
/// # Example
/// ## Request
/// ```sh
/// curl -X DELETE \
///      -i 'http://127.0.0.1:6969/api/user/delete/me' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4'
/// ```
/// ## Response
/// * If user was deleted: HTTP/1.1 200 OK
/// * If DELETE failed in DB: 500 Internal Server Error
#[delete("/me")]
pub async fn delete_me(user: UserDTO, pool: web::Data<PgPool>) -> Result<HttpResponse, APIError> {
    sqlx::query!(r#"DELETE FROM "User" WHERE id = $1"#, user.id)
        .execute(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().finish())
}

/// `DELETE /api/user/delete/{id}`
/// # Example
/// ## Request
/// ```sh
/// curl -X DELETE \
///      -i 'http://127.0.0.1:6969/api/user/delete/00f045ac-1f4d-4601-b2e3-87476dc462e6' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4'
/// ```
/// ## Response
/// * If user was deleted: HTTP/1.1 200 OK
/// * If user was not found: HTTP/1.1 404 Not Found
/// * If DELETE failed in DB: 500 Internal Server Error
#[delete("/{id}")]
pub async fn delete_user(
    id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    // HACK: Use RETURNING * to get error if no user was deleted.
    sqlx::query_as!(
        User,
        r#"
        DELETE FROM "User" 
        WHERE id = $1 
        RETURNING id,
                  username,
                  email,
                  password,
                  first_login,
                  last_login,
                  avatar_url,
                  role AS "role: _",
                  behaviour AS "behaviour: _",
                  is_active,
                  has_verified_email,
                  is_history_private,
                  is_profile_private
        "#,
        id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| UserError::DoesNotExist)?;
    Ok(HttpResponse::Ok().finish())
}

/// `GET /api/user/{id}/peers`
/// # Example
/// ## Request
/// ```sh
/// curl -X GET \
///     -i 'http://127.0.0.1:6969/api/user/00f045ac-1f4d-4601-b2e3-87476dc462e6/peers' \
///     -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///     -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4'
/// ```
/// ## Response
/// HTTP/1.1 200 OK
/// ```json
/// [
///   {
///     "id": "00f045ac-1f4d-4601-b2e3-87476dc462e6",
///     "md5_hash": "aae0bfbf0b0b0b0b0b0b0b0b0b0b0b0b", // md5
///     "info_hash": "afaf9284efc8fae8f8a8f8a8f8a8f8a8", // sha-256
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
///     "torrent_id": "00f045ac-1f4d-4601-b2e3-87476dc462e7"
///   },
///   {
///     ...
///   }
/// ]
/// ```
#[get("/{id}/peers")]
pub async fn get_user_peers(
    id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    let peers = sqlx::query_as!(
        Peer,
        r#"
            SELECT id,
                   md5_hash,
                   info_hash,
                   ip,
                   port,
                   agent,
                   uploaded_bytes,
                   downloaded_bytes,
                   left_bytes,
                   behaviour AS "behaviour: _",
                   created_at,
                   updated_at,
                   user_id,
                   torrent_id
            FROM "Peer" 
            WHERE user_id = $1
            "#,
        id.into_inner()
    )
    .fetch_all(pool.get_ref())
    .await?
    .into_iter()
    .map(PeerDTO::from)
    .collect::<Vec<PeerDTO>>();
    Ok(HttpResponse::Ok().json(peers))
}
