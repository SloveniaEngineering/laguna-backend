use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Event type requested by torrent client via [`Announce`] request.
/// Absence of event means [`AnnounceEvent::Empty`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AnnounceEvent {
  /// [`Peer`] is starting the communication with tracker.
  Started,
  /// [`Peer`] is stopping the communication (could be forced).
  Stopped,
  /// [`Peer`] has completed downloading the file, now it will stop.
  Completed,
  /// [`Peer`] is briefing tracker about his progress.
  Updated,
  /// [`Peer`] has paused downloading but will probably resume (due to wifi issues or force pause).
  Paused,
  /// Treated just like Update, but wasn't specified.
  Empty,
}
