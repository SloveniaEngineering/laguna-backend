use actix_jwt_auth_middleware::TokenSigner;
use actix_web::{web, HttpResponse};

use jwt_compact::alg::Hs256;
use laguna_backend_dto::role::RoleChangeDTO;
use laguna_backend_dto::torrent::TorrentDTO;
use laguna_backend_dto::user::UserDTO;
use laguna_backend_dto::user::UserPatchDTO;
use laguna_backend_middleware::consts::{ACCESS_TOKEN_HEADER_NAME, REFRESH_TOKEN_HEADER_NAME};
use laguna_backend_middleware::mime::APPLICATION_LAGUNA_JSON_VERSIONED;
use laguna_backend_model::behaviour::Behaviour;
use laguna_backend_model::genre::Genre;
use laguna_backend_model::role::Role;
use laguna_backend_model::speedlevel::SpeedLevel;
use laguna_backend_model::torrent::Torrent;
use laguna_backend_model::user::User;
use laguna_backend_model::user::UserSafe;

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{user::UserError, APIError};

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

#[utoipa::path(
  get,
  path = "/api/user/{id}",
  responses(
    (status = 200, description = "Returns user.", body = UserDTO, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    (status = 400, description = "User not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
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
    .map(UserSafe::from)
    .map(UserDTO::from)
    .ok_or(UserError::NotFound)?;
  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(user),
  )
}

#[utoipa::path(
  delete,
  path = "/api/user/{id}",
  responses(
    (status = 200, description = "Delete successful"),
    (status = 400, description = "User not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
  ),
)]
pub async fn user_me_delete(
  user: UserDTO,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  sqlx::query_file_as!(User, "queries/user_delete.sql", user.id)
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserSafe::from)
    .map(drop) // Zero-ize immediately
    .ok_or(UserError::NotFound)?;
  Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
  patch,
  path = "/api/user/me",
  responses(
    (status = 200, description = "Returns updated user.", body = UserDTO, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json", headers(
      ("X-Access-Token" = String, description = "New access token."),
      ("X-Refresh-Token" = String, description = "New refresh token.")
    )),
    (status = 400, description = "User not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
  ),
  request_body = UserPatchDTO
)]
pub async fn user_patch_me(
  user_patch_dto: web::Json<UserPatchDTO>,
  user: UserDTO,
  pool: web::Data<PgPool>,
  signer: web::Data<TokenSigner<UserDTO, Hs256>>,
) -> Result<HttpResponse, APIError> {
  let user = sqlx::query_file_as!(
    User,
    "queries/user_update.sql",
    user_patch_dto.username,
    user_patch_dto.avatar_url,
    user_patch_dto.is_profile_private,
    user.id
  )
  .fetch_optional(pool.get_ref())
  .await?
  .map(UserSafe::from)
  .map(UserDTO::from)
  .ok_or(UserError::NotUpdated)?;
  Ok(
    HttpResponse::Ok()
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

#[utoipa::path(
  patch,
  path = "/api/user/{id}",
  responses(
    (status = 200, description = "Returns updated user.", body = UserDTO, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    (status = 400, description = "User not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
  ),
  request_body = UserPatchDTO,
  params(
    ("id", Path, description = "User's id.", format = Uuid)
  )
)]
pub async fn user_patch(
  _user_id: web::Path<Uuid>,
  _user_patch_dto: web::Json<UserPatchDTO>,
  _current_user: UserDTO,
  _signer: web::Data<TokenSigner<UserDTO, Hs256>>,
  _pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
  patch,
  path = "/api/user/{id}/role_change",
  responses(
    (status = 200, description = "Returns updated user.", body = UserDTO, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    (status = 400, description = "User not found or role not changed due to DB related reasons.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    (status = 403, description = "Not allowed to change role.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
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
    .map(UserSafe::from)
    .ok_or(UserError::NotFound)?;
  // TODO: Can user change its own role? Currently yes but only according to the formula below, which is safe.
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
      .map(UserSafe::from)
      .ok_or(UserError::NotUpdated)?;
      Ok(
        HttpResponse::Ok()
          .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
          .json(UserDTO::from(changed)),
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

/*
Turns out to do this you need to send Auth data to tracker via, say, qBitTorrent which is not possible.
#[utoipa::path(
  get,
  path = "/api/user/{id}/peers",
  responses(
    (status = 200, description = "Returns user's peers.", body = Vec<Peer>, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    (status = 400, description = "User not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
  ),
  params(
    ("id", Path, description = "User's id.", format = Uuid)
  )
)]
pub async fn user_peers_get(
  id: web::Path<Uuid>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let peers = sqlx::query_file_as!(Peer, "queries/user_peers.sql", id.into_inner())
    .fetch_all(pool.get_ref())
    .await?
    .into_iter()
    .collect::<Vec<PeerDTO>>();
  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(peers),
  )
}
*/

#[utoipa::path(
  get,
  path = "/api/user/{id}/torrents",
  responses(
    (status = 200, description = "Returns user's torrents.", body = Vec<Torrent>, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    (status = 400, description = "User not found.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    (status = 401, description = "Not logged in, hence unauthorized.", body = String, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
  ),
  params(
    ("id", Path, description = "User's id.", format = Uuid)
  )
)]
pub async fn user_torrents_get(
  id: web::Path<Uuid>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let torrents = sqlx::query_file_as!(Torrent, "queries/user_torrents.sql", id.into_inner())
    .fetch_all(pool.get_ref())
    .await?
    .into_iter()
    .map(TorrentDTO::from)
    .collect::<Vec<TorrentDTO>>();

  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(torrents),
  )
}
