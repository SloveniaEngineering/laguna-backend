use actix_jwt_auth_middleware::TokenSigner;

use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use jwt_compact::alg::Hs256;
use laguna_backend_dto::login::LoginDTO;
use laguna_backend_dto::user::UserDTO;
use laguna_backend_middleware::{
  consts::{ACCESS_TOKEN_HEADER_NAME, REFRESH_TOKEN_HEADER_NAME},
  mime::APPLICATION_LAGUNA_JSON_VERSIONED,
};
use laguna_backend_model::behaviour::Behaviour;
use laguna_backend_model::role::Role;
use laguna_backend_model::user::User;

use sqlx::PgPool;

use crate::error::{user::UserError, APIError};

#[allow(missing_docs)]
#[utoipa::path(
  post,
  path = "/api/user/auth/login",
  responses(
    (status = 200, description = "User logged in successfully.", body = UserDTO, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json", headers(
      ("X-Access-Token" = String, description = "Access token."),
      ("X-Refresh-Token" = String, description = "Refresh token.")
    )),
    (status = 400, description = "Bad request.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    (status = 401, description = "Invalid credentials.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json")
  ),
)]
pub async fn login(
  login_dto: Json<LoginDTO>,
  pool: web::Data<PgPool>,
  signer: web::Data<TokenSigner<UserDTO, Hs256>>,
  argon_context: web::Data<Argon2<'static>>,
) -> Result<HttpResponse, APIError> {
  let user = sqlx::query_file_as!(
    User,
    "queries/user_lookup.sql",
    login_dto.username_or_email,
    login_dto.username_or_email
  )
  .fetch_optional(pool.get_ref())
  .await?
  .ok_or(UserError::InvalidCredentials)?;

  let password_hash = PasswordHash::new(&user.password).unwrap();
  if argon_context
    .verify_password(login_dto.password.as_bytes(), &password_hash)
    .is_err()
  {
    // SECURITY: Don't report only "Password" or "Username" invalid to avoid brute-force attacks.
    return Err(UserError::InvalidCredentials.into());
    // return Ok(HttpResponse::Unauthorized().body("Uporabniško ime ali geslo napačno"));
  }

  // Update last_login
  let user = sqlx::query_file_as!(User, "queries/user_login_update.sql", Utc::now(), user.id)
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserDTO::from)
    .ok_or(UserError::NotUpdated)?;

  Ok(
    HttpResponse::Ok()
      // TODO: get rid of clones
      .append_header((
        ACCESS_TOKEN_HEADER_NAME,
        signer.create_access_header_value(&user.clone())?,
      ))
      .append_header((
        REFRESH_TOKEN_HEADER_NAME,
        signer.create_refresh_header_value(&user.clone())?,
      ))
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(user),
  )
}
