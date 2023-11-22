#[cfg(feature = "testx")]
use fake::Dummy;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Level of speed of downloading content pointed by torrent file.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, sqlx::Type, ToSchema)]
#[cfg_attr(feature = "testx", derive(Dummy))]
pub enum SpeedLevel {
  /// Low speed swarm (many leeches, slow progression via [`Announce`] reported by torrent clients in this swarm).
  Lowspeed,
  /// Medium speed `(low + high) / 2` requirements.
  Mediumspeed,
  /// Typically around 20+ seeds (many seeds, fast progression via [`Announce`] reported by torrent clients in this swarm).
  Highspeed,
}
