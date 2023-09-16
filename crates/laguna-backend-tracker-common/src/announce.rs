use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AnnounceEvent {
  Started,
  Stopped,
  Completed,
  Updated,
  Paused,
  Empty,
}
