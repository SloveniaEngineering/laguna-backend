use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Rating structure retrieved when user would like to inspect the avg rating and how many rated the torrent.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct TorrentRating {
  /// Average torrent rating, computed in DB using `AVG` function.
  pub average: Option<f64>,
  /// Number of users that voted for this particular torrent.
  pub count: Option<i64>,
}
