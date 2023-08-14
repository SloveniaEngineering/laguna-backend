use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::ipnetwork::IpNetwork;
use uuid::Uuid;

use laguna_backend_tracker_common::info_hash::InfoHash;
use laguna_backend_tracker_common::peer::PeerId;

use crate::behaviour::Behaviour;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct Peer {
  pub id: PeerId,
  pub md5_hash: Option<String>,
  /// Foreign key to info_hash on Torrent
  pub info_hash: InfoHash,
  pub ip: Option<IpNetwork>,
  pub port: i32,
  pub agent: Option<String>,
  pub uploaded_bytes: i64,
  pub downloaded_bytes: i64,
  pub left_bytes: i64,
  pub behaviour: Behaviour,
  pub created_at: DateTime<Utc>,
  pub updated_at: Option<DateTime<Utc>>,
  pub user_id: Uuid,
}
