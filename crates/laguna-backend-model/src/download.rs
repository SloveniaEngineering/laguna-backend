use chrono::DateTime;
use chrono::Utc;
use laguna_backend_tracker_common::info_hash::{InfoHash, SHA256_LENGTH};
use serde::{Deserialize, Serialize};
use serde_with::hex::Hex;
use serde_with::serde_as;
use std::fmt;
use std::fmt::Debug;
use utoipa::ToSchema;
use uuid::Uuid;

/// Download hash of a downloaded torrent, this value is injected directly into [`Announce`] URL and is used to identify the [`User`].
#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, sqlx::Type, ToSchema)]
#[sqlx(transparent)]
pub struct DownloadHash(#[serde_as(as = "Hex")] pub [u8; SHA256_LENGTH]);

/// Download record.
/// Created when user downloads a specific torrent.
/// If the same torrent is downloaded twice, the last download's download record is stored.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, ToSchema, sqlx::FromRow)]
pub struct Download<const N: usize> {
  /// Info hash of torrent file being downloaded.
  pub info_hash: InfoHash<N>,
  /// ID of user that downloads the torrent file.
  pub user_id: Uuid,
  /// Timestamp of download (for security purposes)
  pub ts: DateTime<Utc>,
  /// Computed as sha256 of (info_hash, user_id, ts).
  pub down_hash: DownloadHash,
}

impl From<Vec<u8>> for DownloadHash {
  fn from(vec: Vec<u8>) -> Self {
    DownloadHash(<[u8; SHA256_LENGTH]>::try_from(vec.as_slice()).unwrap())
  }
}

impl fmt::Display for DownloadHash {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(
      self
        .0
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
        .as_str(),
    )
  }
}

impl Debug for DownloadHash {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!("{}", self))
  }
}
