use crate::speedlevel::SpeedLevel;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct Torrent {
    pub id: Uuid,
    pub announce_url: String,
    pub length: i32,
    pub title: String,
    pub file_name: String,
    pub nfo: Option<String>,
    pub leech_count: i32,
    pub seed_count: i32,
    pub completed_count: i32,
    pub speedlevel: SpeedLevel,
    pub info_hash: String,
    pub uploaded_at: DateTime<Utc>,
    pub uploaded_by: Uuid,
    pub modded_at: Option<DateTime<Utc>>,
    pub modded_by: Option<Uuid>,
}
