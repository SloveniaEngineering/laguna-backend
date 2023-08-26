use actix_web::{web, HttpResponse};

use laguna_backend_model::user::UserSafe;
use laguna_backend_model::{peer::Peer, user::User};

use laguna_backend_dto::user::UserDTO;
use laguna_backend_dto::{peer::PeerDTO, user::UserPatchDTO};

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{user::UserError, APIError};

/// `GET /api/user/me`
pub async fn user_me_get(user: UserDTO) -> Result<HttpResponse, APIError> {
  Ok(HttpResponse::Ok().json(user))
}

/// `GET /api/user/{id}`
pub async fn user_get(
  id: web::Path<Uuid>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let user = sqlx::query_as::<_, User>(r#"SELECT * FROM user_get($1)"#)
    .bind(id.into_inner())
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserSafe::from)
    .map(UserDTO::from)
    .ok_or_else(|| UserError::DidntFind)?;
  Ok(HttpResponse::Ok().json(user))
}

/// `DELETE /api/user/me`
pub async fn user_me_delete(
  user: UserDTO,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  sqlx::query_as::<_, User>("SELECT * FROM user_delete($1)")
    .bind(user.id)
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserSafe::from)
    .map(drop) // Zero-ize immediately
    .ok_or_else(|| UserError::DidntFind)?;
  Ok(HttpResponse::Ok().finish())
}

/// `PATCH /api/user/{id}`
pub async fn user_patch(
  user_id: web::Path<Uuid>,
  user_patch_dto: web::Json<UserPatchDTO>,
  current_user: UserDTO,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  // TODO: Should this be middleware?
  let user_id = user_id.into_inner();
  if user_id != current_user.id {
    return Err(UserError::ExclusiveAccess.into());
  }
  let user_patch_dto = user_patch_dto.into_inner();
  let user = sqlx::query_as::<_, User>("SELECT * FROM user_patch($1, $2, $3, $4)")
    .bind(user_id)
    .bind(user_patch_dto.is_history_private)
    .bind(user_patch_dto.is_profile_private)
    .bind(user_patch_dto.avatar_url.unwrap_or_default())
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserSafe::from)
    .map(UserDTO::from)
    .ok_or_else(|| UserError::DidntUpdate)?;
  Ok(HttpResponse::Ok().json(user))
}

/// `GET /api/user/{id}/peers`
pub async fn user_peers_get(
  id: web::Path<Uuid>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let peers = sqlx::query_as::<_, Peer>("SELECT * FROM user_peers_get($1)")
    .bind(id.into_inner())
    .fetch_all(pool.get_ref())
    .await?
    .into_iter()
    .map(PeerDTO::from)
    .collect::<Vec<PeerDTO>>();
  Ok(HttpResponse::Ok().json(peers))
}
