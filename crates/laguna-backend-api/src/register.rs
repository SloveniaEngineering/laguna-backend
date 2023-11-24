use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use argon2::{
  password_hash::{rand_core::OsRng, SaltString},
  Argon2, PasswordHasher,
};

use chrono::Utc;
use laguna_backend_dto::{already_exists::AlreadyExistsDTO, register::RegisterDTO};
use laguna_backend_model::behaviour::Behaviour;
use laguna_backend_model::role::Role;
use laguna_backend_model::user::User;

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
) -> Result<HttpResponse, APIError> {
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

  // TODO: Verify email
  sqlx::query_file_as!(
    User,
    "queries/user_insert.sql",
    register_dto.username,
    register_dto.email,
    password_hash,
    Utc::now(),
    Utc::now(),
    None::<String>,
    salt.to_string(),
    Role::Normie as _,
    0,
    Behaviour::Lurker as _,
    true,
    false,
    false,
    false,
  )
  .fetch_optional(pool.get_ref())
  .await?
  .map(drop)
  .ok_or(UserError::NotCreated)?;

  Ok(HttpResponse::Ok().finish())
}
