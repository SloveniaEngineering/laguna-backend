use crate::genre::Genre;
use crate::speedlevel::SpeedLevel;
use chrono::{DateTime, Utc};

use derivative::Derivative;
use serde::{Deserialize, Serialize};

use laguna_backend_tracker_common::info_hash::{InfoHash, SHA1_LENGTH};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::consts::TORRENT_FILENAME_MAX_LEN;
use crate::consts::TORRENT_FILENAME_MIN_LEN;

use sqlx::FromRow;
use validator::Validate;

#[derive(Derivative, Serialize, Deserialize, PartialEq, Eq, Clone, FromRow, Validate, ToSchema)]
#[derivative(Debug)]
pub struct Torrent {
  pub info_hash: InfoHash<SHA1_LENGTH>,
  #[derivative(Debug = "ignore")]
  pub raw: Vec<u8>,
  pub announce_url: Option<String>,
  pub length: i64,
  #[validate(
    non_control_character,
    length(min = "TORRENT_FILENAME_MIN_LEN", max = "TORRENT_FILENAME_MAX_LEN")
  )]
  pub file_name: String,
  pub nfo: Option<String>,
  pub is_freeleech: bool,
  pub genre: Option<Genre>,
  pub leech_count: i32,
  pub seed_count: i32,
  pub completed_count: i32,
  pub speedlevel: SpeedLevel,
  pub creation_date: DateTime<Utc>,
  pub created_by: Option<String>,
  pub uploaded_at: DateTime<Utc>,
  pub uploaded_by: Uuid,
  pub modded_at: Option<DateTime<Utc>>,
  pub modded_by: Option<Uuid>,
}
