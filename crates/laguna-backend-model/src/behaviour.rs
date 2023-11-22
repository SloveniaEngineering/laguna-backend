use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Behaviour of [`User`] or [`Peer`].
/// In case of [`User`] is just means avergae behaviour over all his [`Peer`]s.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash, sqlx::Type, ToSchema)]
pub enum Behaviour {
  /// No downloads, no seeding nor leeching, almost inactive or inactive deadbeef user.
  Lurker,
  /// Downloads but never completes.
  Downloader,
  /// Leeches but only on freeleech marked torrents.
  Freeleecher,
  /// Downloads, completes and then H&R.
  /// Sufficiently many H&Rs is also a reason for [`Behaviour::Leech`].
  Leech,
  /// Downloads, completes and seeds for other [`Peer`]s for a good amount of time so it is not counted as H&R.
  Seed,
  /// Downloads but doesn't complete due to his shitty wifi.
  Choked,
  /// Torrent uploader. Obtained by uploading and making torrents.
  Uploader,
  /// Downloads but doesn't complete due to his own stopping of downloading progress.
  Stopped,
}
