use actix_web::{web, HttpResponse};

use laguna_backend_model::user::UserSafe;
use laguna_backend_model::{peer::Peer, user::User};

use laguna_backend_dto::user::UserDTO;
use laguna_backend_dto::{peer::PeerDTO, user::UserPatchDTO};

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
/// |Response|Description|
/// |--------|-----------|
/// |200 OK  |User was found. Returns [`UserDTO`]|
/// |401 Unauthorized|Authentication/Authorization middleware failed to authenticate user|
pub async fn user_me_get(user: UserDTO) -> Result<HttpResponse, APIError> {
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
/// |Response|Description|
/// |--------|-----------|
/// |200 OK|User was found. Returns [`UserDTO`]|
/// |400 Bad Request|User was not found|
/// |401 Unauthorized|Authentication/Authorization middleware failed for user requesting this action|
pub async fn user_get(
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
                     salt,
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
    .map(UserSafe::from)
    .map(UserDTO::from)
    .ok_or_else(|| UserError::DoesNotExist)?;
    Ok(HttpResponse::Ok().json(user))
}

/// `DELETE /api/user/me`
/// # Example
/// ## Request
/// ```sh
/// curl -X DELETE \
///      -i 'http://127.0.0.1:6969/api/user/me' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4'
/// ```
/// ## Response
/// |Response|Description|
/// |--------|-----------|
/// |200 OK|User was deleted|
/// |400 Bad Request|User was not found|
/// |401 Unauthorized|Authentication/Authorization middleware failed for user requesting this action|
pub async fn user_me_delete(
    user: UserDTO,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
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
                  salt,
                  role AS "role: _",
                  behaviour AS "behaviour: _",
                  is_active,
                  has_verified_email,
                  is_history_private,
                  is_profile_private
        "#,
        user.id
    )
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserSafe::from)
    .map(drop) // Don't keep user in stack
    .ok_or_else(|| UserError::DoesNotExist)?;
    Ok(HttpResponse::Ok().finish())
}

/// `DELETE /api/user/{id}`
/// # Example
/// ## Request
/// ```sh
/// curl -X DELETE \
///      -i 'http://127.0.0.1:6969/api/user/00f045ac-1f4d-4601-b2e3-87476dc462e6' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4'
/// ```
/// ## Response
/// |Response|Description|
/// |--------|-----------|
/// |200 OK|User was deleted|
/// |401 Unauthorized|Authentication/Authorization middleware failed for user requesting this action|
/// |400 Bad Request|User was not found but permissions are sufficient|
pub async fn user_delete(
    id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
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
                  salt,
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
    .map(UserSafe::from)
    .map(drop) // Don't keep user in stack
    .ok_or_else(|| UserError::DoesNotExist)?;
    Ok(HttpResponse::Ok().finish())
}

/// `PATCH /api/user/me`
/// # Example
/// ## Request
/// ```sh
/// curl -X PATCH \
///     -i 'http://127.0.0.1:6969/api/user/me' \
///     -H 'Content-Type: application/json' \
///     -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg5OTMwNTksImlhdCI6MTY4ODk5Mjk5OSwiaWQiOiIwMGYwNDVhYy0xZjRkLTQ2MDEtYjJlMy04NzQ3NmRjNDYyZTYiLCJ1c2VybmFtZSI6InRlc3QiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDctMTBUMTI6NDI6MzIuMzk2NjQ3WiIsImxhc3RfbG9naW4iOiIyMDIzLTA3LTEwVDEyOjQzOjE5LjIxNjA0N1oiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.izClLn6kANl2kpIv2QqzmKJy7tmpNZqKKvcd4RoGW_c' \
///     -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4' \
/// ```
/// ## Response
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
/// |Response|Description|
/// |--------|-----------|
/// |200 OK|User was updated. Returns updated [`UserDTO`]|
/// |400 Bad Request|User was not found but permissions are sufficient|
/// |401 Unauthorized|Authentication/Authorization middleware failed for user requesting this action|
/// |403 Forbidden|User requesting this action is trying to patch a different user (not themself)|
pub async fn user_patch(
    user_patch_dto: UserPatchDTO,
    current_user: UserDTO,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    // TODO: Should this be middleware?
    if user_patch_dto.id != current_user.id {
        return Err(UserError::ExclusiveAccess.into());
    }
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE "User"
        SET username = $1,
            avatar_url = $2,
            role = $3,
            is_active = $4,
            is_history_private = $5,
            is_profile_private = $6
        WHERE id = $7
        RETURNING id,
                  username,
                  email,
                  password,
                  first_login,
                  last_login,
                  avatar_url,
                  salt,
                  role AS "role: _",
                  behaviour AS "behaviour: _",
                  is_active,
                  has_verified_email,
                  is_history_private,
                  is_profile_private
        "#,
        user_patch_dto.username,
        user_patch_dto.avatar_url,
        user_patch_dto.role as _,
        user_patch_dto.is_active,
        user_patch_dto.is_history_private,
        user_patch_dto.is_profile_private,
        user_patch_dto.id
    )
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserSafe::from)
    .map(UserDTO::from)
    .ok_or_else(|| UserError::DoesNotExist)?;
    Ok(HttpResponse::Ok().json(user))
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
/// |Response|Description|
/// |--------|-----------|
/// |200 OK|[`Vec<PeerDTO>`] were found, if none were found returns empty [`Vec`]|
/// |401 Unauthorized|Authentication/Authorization middleware failed to authenticate user|
pub async fn user_peers_get(
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
