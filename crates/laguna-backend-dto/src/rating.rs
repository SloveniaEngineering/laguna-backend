use laguna_backend_model::rating::Rating;
use laguna_backend_tracker_common::info_hash::InfoHash;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RatingDTO<const N: usize> {
  #[validate(range(min = 0, max = 10))]
  pub rating: i32,
  pub info_hash: InfoHash<N>,
}

impl<const N: usize> From<Rating<N>> for RatingDTO<N> {
  fn from(value: Rating<N>) -> Self {
    Self {
      rating: value.rating,
      info_hash: value.info_hash,
    }
  }
}
