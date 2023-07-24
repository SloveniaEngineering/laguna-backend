use actix_web::post;
use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use digest::Digest;

use laguna_backend_model::{register::RegisterDTO, user::User};
use sha2::Sha256;
use sqlx::PgPool;

use crate::error::APIError;

/// `POST /api/user/auth/register`
/// # Example
/// ### Request
/// ```bash
/// curl -X POST
///      -H "Content-Type: application/json" \
///      -i 'http://127.0.0.1:6969/api/user/auth/register' \
///      --data '{
///         "username": "test",
///         "email": "test@laguna.io",
///         "password": "test123",
///      }'
/// ```
/// ### Response
/// 1. On successful register: HTTP/1.1 200 OK
/// 2. On already registered: HTTP/1.1 208 Already Reported
/// 3. On invalid format (ie. too long, too short, not email, etc.): HTTP/1.1 400 Bad Request
#[post("/register")]
pub async fn register(
    register_dto: Json<RegisterDTO>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    let fetched_user = sqlx::query_as!(
        User,
        r#"
            SELECT id,
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
            FROM "User" 
            WHERE username = $1 OR email = $2"#,
        register_dto.username,
        register_dto.email
    )
    .fetch_optional(pool.get_ref())
    .await?;

    if let Some(_) = fetched_user {
        return Ok(HttpResponse::AlreadyReported().finish());
    }
    // TODO: Verify email
    sqlx::query!(
        r#"
        INSERT INTO "User" (username, email, password)
        VALUES ($1, $2, $3);
    "#,
        register_dto.username,
        register_dto.email,
        format!("{:x}", Sha256::digest(&register_dto.password))
    )
    .execute(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().finish())
}
