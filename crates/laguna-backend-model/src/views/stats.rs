use crate::role::Role;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PeerStats {
  pub downloaded_total: Option<i64>,
  pub uploaded_total: Option<i64>,
  pub left_total: Option<i64>,
  pub peers_total: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TorrentStats {
  pub bytes_total: Option<i64>,
  pub torrents_total: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserStats {
  pub role: Option<Role>,
  pub users_total: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JointStats {
  pub peer_stats: PeerStats,
  pub torrent_stats: TorrentStats,
  pub user_stats: UserStats,
}
