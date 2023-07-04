use actix_web::post;
use actix_web::{web, HttpResponse};
use digest::Digest;

use laguna_backend_model::user::{User, UserDTO};
use sha2::Sha256;
use sqlx::PgPool;

use crate::error::APIError;
use crate::state::UserState;

/// `POST /register`
/// # Example
/// ### Request
/// ```bash
/// curl -X POST -i 'http://127.0.0.1:8080/register' \
///  -H "Content-Type: application/json" \
///  --data '{
///    "username": "test",
///    "email": "test@laguna.io",
///    "password": "test123",
///    "first_login": null,
///    "last_login": null,
///    "avatar_url": null,
///    "role": "Admin",
///    "is_active": null,
///    "has_verified_email": null,
///    "is_history_private": null,
///    "is_profile_private": null
///  }'
/// ```
/// ### Response (on successful register)
/// ```text
/// RegistrationSuccess
/// ```
/// ### Response (on already registered)
/// ```json
/// {
///     "AlreadyRegistered": {
///         "user": {
///             "username": "test",
///             "email": "test@laguna.io",
///             "password": "ecd71870d1963316a97e3ac3408c9835ad8cf0f3c1bc703527c30265534f75ae",
///             "first_login": "2023-07-04T10:18:17.391698Z",
///             "last_login": "2023-07-04T10:18:17.391698Z",
///             "avatar_url": null,
///             "role": "Admin",
///             "is_active": true,
///             "has_verified_email": false,
///             "is_history_private": true,
///             "is_profile_private": true
///         }
///     }
/// }
/// ```
#[post("/register")]
pub async fn register(
    user_dto: web::Json<UserDTO>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    let fetched_user =
        sqlx::query_as::<_, User>("SELECT * FROM \"User\" WHERE username = $1 AND email = $2")
            .bind(&user_dto.username)
            .bind(&user_dto.email)
            .fetch_optional(pool.get_ref())
            .await?;

    if let Some(registered_user) = fetched_user {
        return Ok(
            HttpResponse::AlreadyReported().json(UserState::AlreadyRegistered {
                user: registered_user.into(),
            }),
        );
    }
    // TODO: Verify email
    sqlx::query(
        r#"
        INSERT INTO "User" (username, email, password, role)
        VALUES ($1, $2, $3, $4);
    "#,
    )
    .bind(&user_dto.username)
    .bind(&user_dto.email)
    .bind(format!("{:x}", Sha256::digest(&user_dto.password)))
    .bind(&user_dto.role)
    .execute(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(UserState::RegistrationSuccess))
}
