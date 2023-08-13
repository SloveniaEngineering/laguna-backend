use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use argon2::{
  password_hash::{rand_core::OsRng, SaltString},
  Argon2, PasswordHasher,
};

use laguna_backend_dto::register::RegisterDTO;
use laguna_backend_model::user::{User, UserSafe};

use sqlx::PgPool;

use crate::error::{user::UserError, APIError};

/// `POST /api/user/auth/register`
/// Registers new user.
/// For login see [`login`](crate::login::login).
/// # Example
/// ### Request
/// ```bash
/// curl -X POST \
///      -H "Content-Type: application/json" \
///      -i 'http://127.0.0.1:6969/api/user/auth/register' \
///      --data '{
///         "username": "test123",
///         "email": "test123@laguna.io",
///         "password": "test123"
///      }'
/// ```
/// ### Response
/// #### Status Code
/// |Response|Description|
/// |---|---|
/// |200 OK|Successful register|
/// |208 Already Reported|User already exists|
/// |400 Bad Request|User was not created due to invalid input data|
pub async fn register(
  register_dto: Json<RegisterDTO>,
  pool: web::Data<PgPool>,
  argon_context: web::Data<Argon2<'static>>,
) -> Result<HttpResponse, APIError> {
  // In own scope for faster drop of fetched_user, because we don't need it much.
  let register_dto = register_dto.into_inner();

  {
    let fetched_user = sqlx::query_as::<_, User>("SELECT * FROM user_lookup($1, $2)")
      .bind(&register_dto.username)
      .bind(&register_dto.email)
      .fetch_optional(pool.get_ref())
      .await?
      .map(UserSafe::from);

    if let Some(_) = fetched_user {
      return Ok(HttpResponse::AlreadyReported().finish());
    }
  }

  let salt = SaltString::generate(&mut OsRng);
  let password_hash = argon_context
    .hash_password(register_dto.password.as_bytes(), salt.as_salt())
    .unwrap()
    .to_string();

  // TODO: Verify email
  sqlx::query_as::<_, User>("SELECT * FROM user_insert($1, $2, $3, $4)")
    .bind(register_dto.username)
    .bind(register_dto.email)
    .bind(password_hash)
    .bind(salt.to_string())
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| UserError::DidntCreate)?;

  Ok(HttpResponse::Ok().finish())
}
