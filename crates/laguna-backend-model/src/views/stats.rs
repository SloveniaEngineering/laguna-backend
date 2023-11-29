use crate::role::Role;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Total statistics concerning all [`Peer`]s over all [`Torrent`]s.
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PeerStats {
  pub downloaded_total: Option<i64>,
  pub uploaded_total: Option<i64>,
  pub left_total: Option<i64>,
  pub peers_total: Option<i64>,
}

/// Total statistics concerning all [`Torrent`]s.
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TorrentStats {
  pub bytes_total: Option<i64>,
  pub torrents_total: Option<i64>,
}

/// Total statistics concerning all [`User`]s.
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserStats {
  pub role: Option<Role>,
  pub users_total: Option<i64>,
}

/// Total statistics of all statistics.
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JointStats {
  pub peer_stats: PeerStats,
  pub torrent_stats: TorrentStats,
  pub user_stats: UserStats,
}
