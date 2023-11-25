use std::time::Duration;

use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use argon2::{
  password_hash::{rand_core::OsRng, SaltString},
  Argon2, PasswordHasher,
};

use chrono::Utc;
use digest::Digest;
use laguna_backend_config::get_settings;
use laguna_backend_dto::{already_exists::AlreadyExistsDTO, register::RegisterDTO};
use laguna_backend_model::behaviour::Behaviour;
use laguna_backend_model::role::Role;
use laguna_backend_model::user::User;

use postmark::api::Body;
use postmark::{api::email::SendEmailRequest, reqwest::PostmarkClient, Query};
use sha2::Sha256;
use sqlx::PgPool;

use crate::{
  error::{user::UserError, APIError},
  helpers::register::generate_username_recommendations,
};

#[allow(missing_docs)]
#[utoipa::path(
  post,
  path = "/api/user/auth/register",
  responses(
    (status = 200, description = "User registered successfully."),
    (status = 208, description = "User already exists.", body = AlreadyExistsDTO, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
    (status = 400, description = "Bad request.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
  ),
)]
pub async fn register(
  register_dto: Json<RegisterDTO>,
  pool: web::Data<PgPool>,
  argon_context: web::Data<Argon2<'static>>,
  mailer: web::Data<PostmarkClient>,
) -> Result<HttpResponse, APIError> {
  let registration_begin = Utc::now();
  let register_dto = register_dto.into_inner();

  let fetched_user = sqlx::query_file_as!(
    User,
    "queries/user_lookup.sql",
    register_dto.username,
    register_dto.email
  )
  .fetch_optional(pool.get_ref())
  .await?;

  if let Some(user) = fetched_user {
    return Ok(HttpResponse::AlreadyReported().json(AlreadyExistsDTO {
      message: String::from(
        "Uporabnik s tem uporabniškim imenom, elektronskim naslovom že obstaja.",
      ),
      recommended_usernames: if user.email == register_dto.email {
        Vec::new()
      } else {
        generate_username_recommendations(user, &pool).await?
      },
    }));
  }

  let salt = SaltString::generate(&mut OsRng);
  let password_hash = argon_context
    .hash_password(register_dto.password.as_bytes(), salt.as_salt())
    .unwrap()
    .to_string();

  let email_confirm_expiry = registration_begin + Duration::from_secs(10 * 60); // 10 minutes to confirm email

  let email_confirm_hash = Sha256::digest(format!(
    "{}{}",
    email_confirm_expiry.to_string(),
    register_dto.email.clone()
  ));

  let email_confirm_url = format!(
    "{}/api/email_confirm/{:02x}",
    get_settings().actix.hosts[0].host,
    email_confirm_hash
  );

  let email_req = SendEmailRequest::builder()
    .from(get_settings().application.mailer.sender_email)
    .to(register_dto.email.clone())
    .subject("Potrditev elektronskega naslova")
    .body(Body::html(format!(
      "Za potrditev elektronskega naslova kliknite na <a href=\"{}\">link tukaj</a>.",
      email_confirm_url
    )))
    .build();

  // TODO: Handle email sending errors
  // Right now we are failing silently
  let _email_resp = email_req.execute(mailer.as_ref()).await;

  sqlx::query_file_as!(
    User,
    "queries/user_insert.sql",
    register_dto.username,
    register_dto.email,
    password_hash,
    registration_begin,
    registration_begin,
    None::<String>,
    salt.to_string(),
    Role::Normie as _,
    0,
    Behaviour::Lurker as _,
    true,
    false,
    false, // store unverified email and send pending email confirmation (above)
    false,
    format!("{:02x}", email_confirm_hash),
    email_confirm_expiry,
  )
  .fetch_optional(pool.get_ref())
  .await?
  .map(drop)
  .ok_or(UserError::NotCreated)?;

  Ok(HttpResponse::Ok().finish())
}
