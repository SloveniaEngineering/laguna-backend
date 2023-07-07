use actix_web::post;
use actix_web::{web, HttpResponse};
use digest::Digest;

use laguna_backend_model::{register::RegisterDTO, user::User};
use sha2::Sha256;
use sqlx::PgPool;

use crate::error::APIError;
use crate::state::UserState;

/// `POST /api/user/auth/register`
/// # Example
/// ### Request
/// ```bash
/// curl -X POST -i 'http://127.0.0.1:8080/register' \
///  -H "Content-Type: application/json" \
///  --data '{
///    "username": "test",
///    "email": "test@laguna.io",
///    "password": "test123",
///  }'
/// ```
/// ### Response (on successful register)
/// HTTP/1.1 200 OK
/// ```text
/// RegistrationSuccess
/// ```
/// ### Response (on already registered)
/// HTTP/1.1 208 Already Reported
/// ```text
/// AlreadyRegistered
/// ```
#[post("/register")]
pub async fn register(
    register_dto: web::Json<RegisterDTO>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    let fetched_user =
        sqlx::query_as::<_, User>("SELECT * FROM \"User\" WHERE username = $1 AND email = $2")
            .bind(&register_dto.username)
            .bind(&register_dto.email)
            .fetch_optional(pool.get_ref())
            .await?;

    if let Some(_) = fetched_user {
        return Ok(HttpResponse::AlreadyReported().json(UserState::AlreadyRegistered));
    }
    // TODO: Verify email
    sqlx::query(
        r#"
        INSERT INTO "User" (username, email, password)
        VALUES ($1, $2, $3);
    "#,
    )
    .bind(&register_dto.username)
    .bind(&register_dto.email)
    .bind(format!("{:x}", Sha256::digest(&register_dto.password)))
    .execute(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(UserState::RegistrationSuccess))
}
