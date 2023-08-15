use crate::speedlevel::SpeedLevel;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use laguna_backend_tracker_common::info_hash::InfoHash;
use uuid::Uuid;

use crate::consts::TORRENT_FILENAME_MAX_LEN;
use crate::consts::TORRENT_FILENAME_MIN_LEN;
use crate::consts::TORRENT_TITLE_MAX_LEN;
use crate::consts::TORRENT_TITLE_MIN_LEN;

use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, FromRow, Validate)]
pub struct Torrent {
  pub info_hash: InfoHash,
  pub announce_url: Option<String>,
  pub length: i64,
  #[validate(
    non_control_character,
    length(min = "TORRENT_TITLE_MIN_LEN", max = "TORRENT_TITLE_MAX_LEN")
  )]
  pub title: String,
  #[validate(
    non_control_character,
    length(min = "TORRENT_FILENAME_MIN_LEN", max = "TORRENT_FILENAME_MAX_LEN")
  )]
  pub file_name: String,
  pub nfo: Option<String>,
  pub leech_count: i32,
  pub seed_count: i32,
  pub completed_count: i32,
  pub speedlevel: SpeedLevel,
  pub uploaded_at: DateTime<Utc>,
  pub uploaded_by: Uuid,
  pub modded_at: Option<DateTime<Utc>>,
  pub modded_by: Option<Uuid>,
}
