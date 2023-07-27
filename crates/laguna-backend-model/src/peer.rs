use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::behaviour::Behaviour;


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct Peer {
    pub id: Uuid,
    pub md5_hash: Option<String>,
    pub info_hash: Option<String>,
    pub ip: Option<String>,
    pub port: i32,
    pub agent: Option<String>,
    pub uploaded_bytes: Option<i32>,
    pub downloaded_bytes: Option<i32>,
    pub left_bytes: Option<i32>,
    pub behaviour: Behaviour,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub user_id: Uuid,
    pub torrent_id: Uuid,
}

// For now.
pub type PeerDTO = Peer;
