use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct TorrentRating {
  pub average: Option<f64>,
  pub count: Option<i64>,
}
