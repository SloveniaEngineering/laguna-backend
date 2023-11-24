use crate::error::APIError;
use actix_web::{web, HttpResponse};
use laguna_backend_middleware::mime::APPLICATION_LAGUNA_JSON_VERSIONED;
use laguna_backend_model::role::Role;
use laguna_backend_model::views::stats::{JointStats, PeerStats, TorrentStats, UserStats};
use sqlx::PgPool;

#[allow(missing_docs)]
#[utoipa::path(
    get,
    path = "/api/stats/peer",
    responses(
        (status = 200, description = "Returns `PeerStats`", body = PeerStats, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    ),
)]
pub async fn stats_peer_get(pool: web::Data<PgPool>) -> Result<HttpResponse, APIError> {
  let mut refresh_transaction = pool.begin().await?;
  sqlx::query_file!("queries/stats_peer_refresh.sql")
    .execute(&mut *refresh_transaction)
    .await?;
  let stats = sqlx::query_file_as!(PeerStats, "queries/stats_peer_get.sql")
    .fetch_one(&mut *refresh_transaction)
    .await?;
  refresh_transaction.commit().await?;
  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(stats),
  )
}

#[allow(missing_docs)]
#[utoipa::path(
    get,
    path = "/api/stats/torrent",
    responses(
        (status = 200, description = "Returns `TorrentStats`", body = TorrentStats, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    ),
)]
pub async fn stats_torrent_get(pool: web::Data<PgPool>) -> Result<HttpResponse, APIError> {
  let mut refresh_transaction = pool.begin().await?;
  sqlx::query_file!("queries/stats_torrent_refresh.sql")
    .execute(&mut *refresh_transaction)
    .await?;
  let stats = sqlx::query_file_as!(TorrentStats, "queries/stats_torrent_get.sql")
    .fetch_one(&mut *refresh_transaction)
    .await?;
  refresh_transaction.commit().await?;
  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(stats),
  )
}

#[allow(missing_docs)]
#[utoipa::path(
    get,
    path = "/api/stats/user",
    responses(
        (status = 200, description = "Returns `UserStats`", body = UserStats, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    ),
)]
pub async fn stats_user_get(pool: web::Data<PgPool>) -> Result<HttpResponse, APIError> {
  let mut refresh_transaction = pool.begin().await?;
  sqlx::query_file!("queries/stats_user_refresh.sql")
    .execute(&mut *refresh_transaction)
    .await?;
  let stats = sqlx::query_file_as!(UserStats, "queries/stats_user_get.sql")
    .fetch_one(&mut *refresh_transaction)
    .await?;
  refresh_transaction.commit().await?;
  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(stats),
  )
}

#[allow(missing_docs)]
#[utoipa::path(
    get,
    path = "/api/stats/",
    responses(
        (status = 200, description = "Returns `JointStats`", body = JointStats, content_type = "application/vnd.sloveniaengineering.laguna.1.0.0-alpha+json"),
    ),
)]
pub async fn stats_joint_get(pool: web::Data<PgPool>) -> Result<HttpResponse, APIError> {
  let mut refresh_transaction = pool.begin().await?;
  sqlx::query_file!("queries/stats_peer_refresh.sql")
    .execute(&mut *refresh_transaction)
    .await?;
  sqlx::query_file!("queries/stats_torrent_refresh.sql")
    .execute(&mut *refresh_transaction)
    .await?;
  sqlx::query_file!("queries/stats_user_refresh.sql")
    .execute(&mut *refresh_transaction)
    .await?;
  let peer_stats = sqlx::query_file_as!(PeerStats, "queries/stats_peer_get.sql")
    .fetch_one(&mut *refresh_transaction)
    .await?;
  let torrent_stats = sqlx::query_file_as!(TorrentStats, "queries/stats_torrent_get.sql")
    .fetch_one(&mut *refresh_transaction)
    .await?;
  let user_stats = sqlx::query_file_as!(UserStats, "queries/stats_user_get.sql")
    .fetch_one(&mut *refresh_transaction)
    .await?;
  refresh_transaction.commit().await?;
  Ok(
    HttpResponse::Ok()
      .content_type(APPLICATION_LAGUNA_JSON_VERSIONED)
      .json(JointStats {
        peer_stats,
        torrent_stats,
        user_stats,
      }),
  )
}
