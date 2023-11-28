use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::ipnetwork::IpNetwork;
use utoipa::ToSchema;

use laguna_backend_tracker_common::info_hash::InfoHash;
use laguna_backend_tracker_common::peer::PeerId;
use uuid::Uuid;

use crate::behaviour::Behaviour;

/// [`Peer`] DB object.
/// A [`Peer`] is created when leeching/seeding or anything else a user does with a **file pointed to by torrent** (except when **torrent file** is downloaded).
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash, sqlx::FromRow, ToSchema)]
pub struct Peer<const N: usize> {
  /// Unique id of Peer.
  pub uuid: Uuid,
  /// Id of Peer as reported by torrent client [`laguna_backend_tracker_common::peer::PeerClient`] (peer client program ie. BitTorrent, uBitTorrent, etc).
  pub id: PeerId,
  /// MD5 of Peer
  pub md5_hash: Option<String>,
  /// Info hash of torrent file this [`Peer`] is interacting with.
  pub info_hash: InfoHash<N>,
  /// IP of this peer.
  pub ip: IpNetwork,
  /// Port of this peer.
  pub port: i32,
  /// Origin is first peer in swarm, usually uploader's peer.
  /// Is this peer origin?
  pub is_origin: bool,
  /// User agent used by this peer.
  pub agent: Option<String>,
  /// How many bytes did this peer send to other peers in swarm?
  pub uploaded_bytes: i64,
  /// How many bytes did this peer receive from other peers in swarm?
  pub downloaded_bytes: i64,
  /// How many bytes more does this peer need to complete?
  pub left_bytes: i64,
  /// What is specific behaviour of this peer, ie. seed/leech/choked/...? Computed.
  pub behaviour: Behaviour,
  /// When was this peer created (first contact with tracker)?
  pub created_at: DateTime<Utc>,
  /// When was this peer updated (last contact with tracker)?
  pub updated_at: Option<DateTime<Utc>>,
  /// Who does this peer belong to? User's ID.
  pub created_by: Uuid,
}
