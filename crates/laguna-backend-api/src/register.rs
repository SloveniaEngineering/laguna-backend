use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, ParamsBuilder, PasswordHasher, Version,
};

use laguna_backend_dto::register::RegisterDTO;
use laguna_backend_model::user::{User, UserSafe};

use sqlx::PgPool;

use crate::error::{user::UserError, APIError};

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
/// 4. On DB operation failure: HTTP/1.1 500 Internal Server Error
pub async fn register(
    register_dto: Json<RegisterDTO>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    // In own scope for faster drop of fetched_user, because we don't need it much.
    {
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
                    salt,
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
        .await?
        .map(UserSafe::from);

        if let Some(_) = fetched_user {
            return Ok(HttpResponse::AlreadyReported().finish());
        }
    }

    // https://github.com/SloveniaEngineering/laguna-backend/issues/54#issuecomment-1645126931
    let argon_context = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        ParamsBuilder::new()
            .p_cost(1)
            .m_cost(12288) // 12MiB in kibibytes
            .t_cost(3)
            .build()
            .unwrap(),
    );

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon_context
        .hash_password(register_dto.password.as_bytes(), salt.as_salt())
        .unwrap()
        .to_string();

    // TODO: Verify email
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO "User" (username, email, password, salt)
        VALUES ($1, $2, $3, $4)
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
                  is_profile_private;
    "#,
        register_dto.username,
        register_dto.email,
        password_hash,
        salt.to_string()
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| UserError::DidntCreate)?;
    Ok(HttpResponse::Ok().finish())
}
