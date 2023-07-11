use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TorrentDTO {
    pub title: String,
    pub file_name: String,
    pub nfo: Option<String>,
    pub info_hash: String,
    pub uploaded_at: Option<DateTime<Utc>>,
    pub uploaded_by: Uuid,
    pub modded_by: Option<Uuid>,
    #[serde(with = "serde_bytes")]
    pub payload: Vec<u8>,
}

impl From<Torrent> for TorrentDTO {
    fn from(torrent: Torrent) -> Self {
        Self {
            title: torrent.title,
            file_name: torrent.file_name,
            nfo: torrent.nfo,
            info_hash: torrent.info_hash,
            uploaded_at: Some(torrent.uploaded_at),
            uploaded_by: torrent.uploaded_by,
            modded_by: torrent.modded_by,
            payload: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TorrentPutDTO {
    pub title: String,
    pub file_name: String,
    pub nfo: Option<String>,
    pub uploaded_by: Uuid,
    pub modded_by: Option<Uuid>,
    #[serde(with = "serde_bytes")]
    pub payload: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TorrentPatchDTO {
    pub title: String,
    pub file_name: String,
    pub nfo: Option<String>,
    pub modded_by: Option<Uuid>,
}
