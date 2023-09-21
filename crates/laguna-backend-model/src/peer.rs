use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::ipnetwork::IpNetwork;
use utoipa::ToSchema;

use laguna_backend_tracker_common::info_hash::{InfoHash, SHA1_LENGTH};
use laguna_backend_tracker_common::peer::PeerId;
use uuid::Uuid;

use crate::behaviour::Behaviour;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash, sqlx::FromRow, ToSchema)]
pub struct Peer {
  pub uuid: Uuid,
  pub id: PeerId,
  pub md5_hash: Option<String>,
  pub info_hash: InfoHash<SHA1_LENGTH>,
  pub ip: IpNetwork,
  pub port: i32,
  // origin is first peer in swarm, usually uploader's peer
  pub is_origin: bool,
  pub agent: Option<String>,
  pub uploaded_bytes: i64,
  pub downloaded_bytes: i64,
  pub left_bytes: i64,
  pub behaviour: Behaviour,
  pub created_at: DateTime<Utc>,
  pub updated_at: Option<DateTime<Utc>>,
}
