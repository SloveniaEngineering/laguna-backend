use actix_web::{web, HttpResponse};

use laguna_backend_dto::peer::PeerDTO;
use laguna_backend_dto::role::RoleChangeDTO;
use laguna_backend_dto::torrent::TorrentDTO;
use laguna_backend_dto::user::UserDTO;

use laguna_backend_dto::user::UserPatchDTO;

use laguna_backend_middleware::mime::APPLICATION_LAGUNA_JSON_VERSIONED;
use laguna_backend_model::behaviour::Behaviour;
use laguna_backend_model::genre::Genre;
use laguna_backend_model::peer::Peer;
use laguna_backend_model::role::Role;
use laguna_backend_model::speedlevel::SpeedLevel;
use laguna_backend_model::torrent::Torrent;
use laguna_backend_model::user::User;

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{user::UserError, APIError};

#[allow(missing_docs)]
#[utoipa::path(
    get,
    path = "/api/user/me",
    responses(
        (status = 200, description = "Returns current user.", body = UserDTO),
        (status = 401, description = "Not logged in, hence unauthorized.", body = String)
    ),
)]
pub async fn user_me_get(user: UserDTO) -> Result<HttpResponse, APIError> {
  Ok(HttpResponse::Ok().json(user))
}

#[allow(missing_docs)]
#[utoipa::path(
  get,
  path = "/api/user/{id}",
  responses(
    (status = 200, description = "Returns user.", body = UserDTO, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
    (status = 400, description = "User not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
  ),
  params(
    ("id", Path, description = "User's id.", format = Uuid)
  )
)]
pub async fn user_get(
  id: web::Path<Uuid>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let user = sqlx::query_file_as!(User, "queries/user_get.sql", id.into_inner())
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserDTO::from)
    .ok_or(UserError::NotFound)?;
  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(user),
  )
}

#[allow(missing_docs)]
#[utoipa::path(
  delete,
  path = "/api/user/{id}",
  responses(
    (status = 200, description = "Delete successful"),
    (status = 400, description = "User not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
  ),
)]
pub async fn user_me_delete(
  user: UserDTO,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  sqlx::query_file_as!(User, "queries/user_delete.sql", user.id)
    .fetch_optional(pool.get_ref())
    .await?
    .map(drop) // Zero-ize immediately
    .ok_or(UserError::NotFound)?;
  Ok(HttpResponse::Ok().finish())
}

#[allow(missing_docs)]
#[utoipa::path(
  patch,
  path = "/api/user/{id}",
  responses(
    (status = 200, description = "Returns updated user.", body = UserDTO, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
    (status = 400, description = "User not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
  ),
  request_body = UserPatchDTO,
  params(
    ("id", Path, description = "User's id.", format = Uuid)
  )
)]
pub async fn user_patch(
  user_id: web::Path<Uuid>,
  user_patch_dto: web::Json<UserPatchDTO>,
  current_user: UserDTO,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  // Only allow self or admin to change user.
  // TODO: Middleware this.
  let user_id = user_id.into_inner();
  if user_id != current_user.id && current_user.role != Role::Admin {
    Err(UserError::ExclusiveOrAdmin)?;
  }

  let user = sqlx::query_file_as!(
    User,
    "queries/user_update.sql",
    user_patch_dto.avatar_url,
    user_patch_dto.is_profile_private,
    user_id
  )
  .fetch_optional(pool.get_ref())
  .await?
  .map(UserDTO::from)
  .ok_or(UserError::NotUpdated)?;
  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(user),
  )
}

#[allow(missing_docs)]
#[utoipa::path(
  patch,
  path = "/api/user/{id}/role_change",
  responses(
    (status = 200, description = "Returns updated user.", body = UserDTO, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
    (status = 400, description = "User not found or role not changed due to DB related reasons.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
    (status = 403, description = "Not allowed to change role.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
  ),
)]
pub async fn user_role_change(
  user_id: web::Path<Uuid>,
  role_change_dto: web::Json<RoleChangeDTO>,
  current_user: UserDTO,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let changee = sqlx::query_file_as!(User, "queries/user_get.sql", user_id.into_inner())
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or(UserError::NotFound)?;
  // User can't change his own role.
  if changee.id == current_user.id {
    Err(UserError::SelfRoleChangeNotAllowed)?;
  }
  match (current_user.role, changee.role, role_change_dto.to) {
    (Role::Admin, _, _)
    | (Role::Mod, Role::Verified | Role::Normie, Role::Verified | Role::Normie) => {
      let changed = sqlx::query_file_as!(
        User,
        "queries/user_role_change.sql",
        role_change_dto.to as _,
        changee.id
      )
      .fetch_optional(pool.get_ref())
      .await?
      .map(UserDTO::from)
      .ok_or(UserError::NotUpdated)?;
      Ok(
        HttpResponse::Ok()
          .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
          .json(changed),
      )
    },
    (changer, changee_from, changee_to) => Err(
      UserError::RoleChangeNotAllowed {
        changer,
        changee_from,
        changee_to,
      }
      .into(),
    ),
  }
}

#[allow(missing_docs)]
#[utoipa::path(
  get,
  path = "/api/user/{id}/peers",
  responses(
    (status = 200, description = "Returns user's peers.", body = Vec<Peer>, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
    (status = 400, description = "User not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
  ),
  params(
    ("id", Path, description = "User's id.", format = Uuid)
  )
)]
pub async fn user_peers_get<const N: usize>(
  id: web::Path<Uuid>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let peers = sqlx::query_file_as!(Peer, "queries/user_peers.sql", id.into_inner())
    .fetch_all(pool.get_ref())
    .await?
    .into_iter()
    .collect::<Vec<PeerDTO<N>>>();
  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(peers),
  )
}

#[allow(missing_docs)]
#[utoipa::path(
  get,
  path = "/api/user/{id}/torrents",
  responses(
    (status = 200, description = "Returns user's torrents.", body = Vec<Torrent>, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
    (status = 400, description = "User not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-beta+json"),
  ),
  params(
    ("id", Path, description = "User's id.", format = Uuid)
  )
)]
pub async fn user_torrents_get<const N: usize>(
  id: web::Path<Uuid>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let torrents = sqlx::query_file_as!(Torrent::<N>, "queries/user_torrents.sql", id.into_inner())
    .fetch_all(pool.get_ref())
    .await?
    .into_iter()
    .map(TorrentDTO::<N>::from)
    .collect::<Vec<TorrentDTO<N>>>();

  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(torrents),
  )
}
