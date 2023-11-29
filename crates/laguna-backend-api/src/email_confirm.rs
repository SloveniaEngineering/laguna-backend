use actix_jwt_auth_middleware::TokenSigner;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use jwt_compact::alg::Hs256;
use laguna_backend_dto::user::UserDTO;
use laguna_backend_middleware::consts::{ACCESS_TOKEN_HEADER_NAME, REFRESH_TOKEN_HEADER_NAME};
use laguna_backend_model::behaviour::Behaviour;
use laguna_backend_model::role::Role;
use laguna_backend_model::user::User;
use sqlx::PgPool;

use crate::error::{user::UserError, APIError};

#[allow(missing_docs)]
#[utoipa::path(
  get,
  path = "/api/email_confirm/{email_confirm_hash}",
  params(
    ("email_confirm_hash", Path, description = "Email confirm hash.", format = Byte)
  )
)]
pub async fn email_confirm(
  current_user: UserDTO,
  email_confirm_hash: web::Path<String>,
  pool: web::Data<PgPool>,
  signer: web::Data<TokenSigner<UserDTO, Hs256>>,
) -> Result<HttpResponse, APIError> {
  let user = sqlx::query_file_as!(
    User,
    "queries/user_lookup_by_email_confirm_hash.sql",
    email_confirm_hash.into_inner()
  )
  .fetch_optional(pool.get_ref())
  .await?
  .ok_or(UserError::NotFound)?;
  if user.id != current_user.id {
    Err(UserError::Exclusive)?;
  }
  if user.has_verified_email {
    return Err(UserError::EmailAlreadyVerified.into());
  }
  // SAFETY: Unwrap is safe because if user has not verified email, email_confirm_expiry is Some.
  if user.email_confirm_expiry.unwrap() < Utc::now() {
    return Err(UserError::EmailConfirmHashExpired.into());
  }
  let user = sqlx::query_file_as!(User, "queries/user_email_confirm.sql", user.id)
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserDTO::from)
    .ok_or(UserError::NotFound)?;
  Ok(
    HttpResponse::Ok()
      .append_header((
        ACCESS_TOKEN_HEADER_NAME,
        signer.create_access_header_value(&user)?,
      ))
      .append_header((
        REFRESH_TOKEN_HEADER_NAME,
        signer.create_refresh_header_value(&user)?,
      ))
      .json(user),
  )
}
