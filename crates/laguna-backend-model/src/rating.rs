use laguna_backend_tracker_common::info_hash::InfoHash;
use serde::{Deserialize, Serialize};

use uuid::Uuid;
use validator::Validate;

/// [`Torrent`]'s single rating by [`User`].
/// This is esentially M2M between [`User`] and [`Torrent`].
#[derive(Debug, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct Rating<const N: usize> {
  /// Rating user gave to this torrent.
  #[validate(range(min = 0, max = 10))]
  pub rating: i32,
  /// Which user rated?
  pub user_id: Uuid,
  /// Which torrent was rated?
  pub info_hash: InfoHash<N>,
}
