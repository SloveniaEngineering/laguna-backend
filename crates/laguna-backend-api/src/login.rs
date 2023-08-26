use actix_jwt_auth_middleware::TokenSigner;

use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use jwt_compact::alg::Hs256;
use laguna_backend_dto::login::LoginDTO;
use laguna_backend_dto::user::UserDTO;
use laguna_backend_middleware::consts::{ACCESS_TOKEN_HEADER_NAME, REFRESH_TOKEN_HEADER_NAME};
use laguna_backend_model::user::{User, UserSafe};

use secrecy::ExposeSecret;
use sqlx::PgPool;

use crate::error::{user::UserError, APIError};

/// `POST /api/user/auth/login`
/// Signs in existing user.
pub async fn login(
  login_dto: Json<LoginDTO>,
  pool: web::Data<PgPool>,
  signer: web::Data<TokenSigner<UserDTO, Hs256>>,
  argon_context: web::Data<Argon2<'static>>,
) -> Result<HttpResponse, APIError> {
  let user = sqlx::query_as::<_, User>("SELECT * FROM user_lookup($1, $1)")
    .bind(&login_dto.username_or_email)
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserSafe::from)
    .ok_or_else(|| UserError::InvalidCredentials)?;

  let password_hash = PasswordHash::new(user.password.expose_secret()).unwrap();
  if let Err(_) = argon_context.verify_password(login_dto.password.as_bytes(), &password_hash) {
    // SECURITY: Don't report only "Password" or "Username" invalid to avoid brute-force attacks.
    return Err(UserError::InvalidCredentials.into());
    // return Ok(HttpResponse::Unauthorized().body("Uporabniško ime ali geslo napačno"));
  }
  // Logged user has been updated, we need to return the updated user.
  // TODO(kenpaicat): This is fine, UTC stamp on backend recieve is fine.
  //                  But we probably should do this on INSERT TRIGGER or as DEFAULT on table.
  let user = sqlx::query_as::<_, User>("SELECT * FROM user_patch_login($1, $2)")
    .bind(user.id)
    .bind(Utc::now())
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserSafe::from)
    .map(UserDTO::from)
    .ok_or_else(|| UserError::DidntUpdate)?;
  Ok(
    HttpResponse::Ok()
      // TODO: get rid of clones
      .append_header((
        ACCESS_TOKEN_HEADER_NAME,
        signer.create_access_header_value(&user.clone().into())?,
      ))
      .append_header((
        REFRESH_TOKEN_HEADER_NAME,
        signer.create_refresh_header_value(&user.clone().into())?,
      ))
      .json(user),
  )
}
