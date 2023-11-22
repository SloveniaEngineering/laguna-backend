use actix_web::{web, HttpResponse};
use laguna_backend_dto::{rating::RatingDTO, user::UserDTO};
use laguna_backend_tracker_common::info_hash::InfoHash;

use laguna_backend_model::rating::Rating;
use laguna_backend_model::torrent_rating::TorrentRating;
use sqlx::PgPool;

use crate::error::rating::RatingError;
use crate::error::APIError;
use actix_web_validator::Json;

#[allow(missing_docs)]
#[utoipa::path(
    post,
    path = "/api/torrent/rating",
    request_body = RatingDTO<N>,
    responses(
        (status = 200, description = "Rating created.", content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
        (status = 400, description = "Rating already exists.", content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
        (status = 401, description = "Not logged in, hence unauthorized.", content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    ),
)]
pub async fn rating_create<const N: usize>(
  rating_dto: Json<RatingDTO<N>>,
  user_dto: UserDTO,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let maybe_rating = sqlx::query_file_as!(
    Rating::<N>,
    "queries/rating_lookup.sql",
    user_dto.id,
    rating_dto.info_hash as _
  )
  .fetch_optional(pool.get_ref())
  .await?;

  if maybe_rating.is_some() {
    return Err(RatingError::AlreadyRated.into());
  }

  let rating_dto = rating_dto.into_inner();
  let _rating = sqlx::query_file_as!(
    Rating::<N>,
    "queries/rating_insert.sql",
    rating_dto.rating,
    user_dto.id,
    rating_dto.info_hash as _
  )
  .fetch_optional(pool.get_ref())
  .await?
  .ok_or(RatingError::NotCreated);
  Ok(HttpResponse::Ok().finish())
}

#[allow(missing_docs)]
#[utoipa::path(
    delete,
    path = "/api/torrent/rating/{info_hash}",
    responses(
        (status = 200, description = "Rating deleted.", content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
        (status = 401, description = "Not logged in, hence unauthorized.", content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    ),
)]
pub async fn rating_delete<const N: usize>(
  info_hash: web::Path<InfoHash<N>>,
  user_dto: UserDTO,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let _rating = sqlx::query_file_as!(
    Rating::<N>,
    "queries/rating_delete.sql",
    user_dto.id,
    info_hash.into_inner() as _
  )
  .fetch_optional(pool.get_ref())
  .await?
  .ok_or(RatingError::NotDeleted)?;
  Ok(HttpResponse::Ok().finish())
}

#[allow(missing_docs)]
#[utoipa::path(
    get,
    path = "/api/torrent/rating/{info_hash}",
    responses(
        (status = 200, description = "Rating for torrent.", body = TorrentRating, content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
        (status = 401, description = "Not logged in, hence unauthorized.", content_type = "application/vnd.sloveniaengineering.laguna.0.1.0+json"),
    ),
)]
pub async fn rating_torrent_average<const N: usize>(
  info_hash: web::Path<InfoHash<N>>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
  let rating = sqlx::query_file_as!(
    TorrentRating,
    "queries/rating_torrent_average.sql",
    info_hash.into_inner() as _
  )
  .fetch_one(pool.get_ref())
  .await?;
  Ok(HttpResponse::Ok().json(rating))
}
