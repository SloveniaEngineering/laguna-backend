use laguna_backend_tracker_common::info_hash::InfoHash;
use serde::{Deserialize, Serialize};

use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct Rating<const N: usize> {
  #[validate(range(min = 0, max = 10))]
  pub rating: i32,
  pub user_id: Uuid,
  pub info_hash: InfoHash<N>,
}
