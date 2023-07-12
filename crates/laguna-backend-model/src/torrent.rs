use crate::consts::{TORRENT_FILENAME_MAX_LEN, TORRENT_FILENAME_MIN_LEN};
use crate::consts::{TORRENT_TITLE_MAX_LEN, TORRENT_TITLE_MIN_LEN};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// http://bittorrent.org/beps/bep_0052.html
/// http://bittorrent.org/beps/bep_0023.html
/// http://bittorrent.org/beps/bep_0007.html
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct Torrent {
    pub id: Uuid,
    pub title: String,
    pub file_name: String,
    pub nfo: Option<String>,
    pub info_hash: String,
    pub uploaded_at: DateTime<Utc>,
    pub uploaded_by: Uuid,
    pub modded_by: Option<Uuid>,
}

#[derive(Serialize, Deserialize)]
pub struct TorrentFileInfo {
    pub name: String,
    #[serde(rename = "piece length")]
    pub piece_length: u64,
    #[serde(rename = "meta version")]
    pub meta_version: u64,
    #[serde(rename = "file tree")]
    pub file_tree: Vec<u8>,
    pub length: u64,
    #[serde(rename = "pieces root")]
    pub pieces_root: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Validate)]
pub struct TorrentDTO {
    pub id: Uuid,
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
    pub info_hash: String,
    pub uploaded_at: DateTime<Utc>,
    pub uploaded_by: Uuid,
    pub modded_by: Option<Uuid>,
}

impl From<Torrent> for TorrentDTO {
    fn from(torrent: Torrent) -> Self {
        Self {
            id: torrent.id,
            title: torrent.title,
            file_name: torrent.file_name,
            nfo: torrent.nfo,
            info_hash: torrent.info_hash,
            uploaded_at: torrent.uploaded_at,
            uploaded_by: torrent.uploaded_by,
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
