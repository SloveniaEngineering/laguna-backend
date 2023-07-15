use crate::consts::{TORRENT_FILENAME_MAX_LEN, TORRENT_FILENAME_MIN_LEN};
use crate::consts::{TORRENT_TITLE_MAX_LEN, TORRENT_TITLE_MIN_LEN};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, sqlx::Type)]
pub enum SpeedLevel {
    Lowspeed,
    Mediumspeed,
    Highspeed
}

/// <http://bittorrent.org/beps/bep_0052.html>
/// <http://bittorrent.org/beps/bep_0023.html>
/// <http://bittorrent.org/beps/bep_0007.html>
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct Torrent {
    pub id: Uuid,
    pub announce_url: String,
    pub size: i64,
    pub title: String,
    pub file_name: String,
    pub nfo: Option<String>,
    pub leech_count: i64,
    pub seed_count: i64,
    pub completed_count: i64,
    pub speedlevel: SpeedLevel,
    pub info_hash: String,
    pub uploaded_at: DateTime<Utc>,
    pub uploaded_by: Uuid,
    pub modded_at: Option<DateTime<Utc>>,
    pub modded_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Validate)]
pub struct TorrentDTO {
    pub id: Uuid,
    pub announce_url: String,
    pub size: i64,
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
    pub leech_count: i64,
    pub seed_count: i64,
    pub completed_count: i64,
    pub speedlevel: SpeedLevel,
    pub info_hash: String,
    pub uploaded_at: DateTime<Utc>,
    pub uploaded_by: Uuid,
    pub modded_at: Option<DateTime<Utc>>,
    pub modded_by: Option<Uuid>,
}

impl From<Torrent> for TorrentDTO {
    fn from(torrent: Torrent) -> Self {
        Self {
            id: torrent.id,
            announce_url: torrent.announce_url,
            size: torrent.size,
            title: torrent.title,
            file_name: torrent.file_name,
            nfo: torrent.nfo,
            leech_count: torrent.leech_count,
            seed_count: torrent.seed_count,
            completed_count: torrent.completed_count,
            speedlevel: torrent.speedlevel,
            info_hash: torrent.info_hash,
            uploaded_at: torrent.uploaded_at,
            uploaded_by: torrent.uploaded_by,
            modded_at: torrent.modded_at,
            modded_by: torrent.modded_by,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Validate)]
pub struct TorrentPutDTO {
    #[validate(length(min = "TORRENT_TITLE_MIN_LEN", max = "TORRENT_TITLE_MAX_LEN"))]
    pub title: String,
    #[validate(length(min = "TORRENT_FILENAME_MIN_LEN", max = "TORRENT_FILENAME_MAX_LEN"))]
    // TODO: Filename validation.
    pub file_name: String,
    pub nfo: Option<String>,
    pub uploaded_by: Uuid,
    pub modded_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Validate)]
pub struct TorrentPatchDTO {
    pub id: Uuid,
    #[validate(length(min = "TORRENT_TITLE_MIN_LEN", max = "TORRENT_TITLE_MAX_LEN"))]
    pub title: String,
    #[validate(length(min = "TORRENT_FILENAME_MIN_LEN", max = "TORRENT_FILENAME_MAX_LEN"))]
    pub file_name: String,
    pub nfo: Option<String>,
    pub modded_by: Option<Uuid>,
}
