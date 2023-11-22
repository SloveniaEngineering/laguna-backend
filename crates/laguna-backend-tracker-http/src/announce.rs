use std::net::IpAddr;

use bendy::encoding::{self, SingleItemEncoder, ToBencode};
use laguna_backend_tracker_common::helpers::bool_from_int;
use laguna_backend_tracker_common::{
  announce::AnnounceEvent,
  info_hash::InfoHash,
  peer::{PeerId, PeerStream},
};

use serde::{Deserialize, Serialize};

use laguna_backend_model::download::DownloadHash;
use utoipa::ToSchema;

/// Used by torrent client to send information to TCP-based torrent tracker.
/// This payload comes via URL parameters.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Announce<const N: usize> {
  /// Download hash created by laguna.
  /// Torrents downloaded from laguna are injected with `download_hash` URL parameter.
  pub down_hash: DownloadHash,
  /// Info hash is computed by torrent client as sha1 or sha256 of `info` section in torrent file.
  /// `sha1` is used for v1 bittorrent, `sha256` is used for v2.
  /// We support both v1 and v2 by using const parameter `N` (size of sha in bytes) determining which version is used.
  /// If `N = 20` then `sha1` and `v1`, if `N = 32` then `sha256` and `v2`.
  pub info_hash: InfoHash<N>,
  /// Peer id is set by torrent client, it may or may not be unique.
  /// It is likely unique for all torrent clients on same network, but it probably isn't for clients from different networks.
  pub peer_id: PeerId,
  /// IP of client, this is optional but some clients send it.
  #[serde(default)]
  pub ip: Option<IpAddr>,
  /// Port of client, required.
  pub port: u16,
  /// Number of bytes "uploaded", meaning sent to other peers.
  pub uploaded: i64,
  /// Number of bytes "downloaded", meaning received from other peers.
  pub downloaded: i64,
  /// Number of bytes missing to complete the download of the file described by torrent.
  pub left: i64,
  /// Type of event this payload represents, if [`None`] it means [`AnnounceEvent::Update`].
  #[serde(default)]
  pub event: Option<AnnounceEvent>,
  /// Number of peers that want to "download" receive from this peer.
  #[serde(default)]
  pub numwant: Option<i64>,
  /// Is this peer ready to receive compact information?
  /// If `Some(true)`, then [`AnnounceReply`]s to will be bencoded strings rather then dictionaries.
  /// If `None` (not specified) or `Some(false)`, [`AnnounceReply`]s will be bencoded dictionaries.
  /// Compact transmission is faster.
  #[serde(default)]
  #[serde(deserialize_with = "bool_from_int")]
  pub compact: Option<bool>,
  /// Set by torrent client to imply that peer id should not be stored ad-hoc or is not reliable.
  #[serde(default)]
  #[serde(deserialize_with = "bool_from_int")]
  pub no_peer_id: Option<bool>,
  /// Special key that if [`no_peer_id`] is specified, should be used as ID.
  pub key: Option<String>,
  /// ID of tracker in multi-tracker configuration.
  /// This is not used by laguna, but is here for future, and also because we don't want parse errors due to possible missing fields.
  pub trackerid: Option<String>,
  /// Does torrent client support libcrypto?
  #[serde(default)]
  #[serde(deserialize_with = "bool_from_int")]
  pub supportcrypto: Option<bool>,
  /// Redundant request from torrent client, usually as a result of user "force announcing" themselves.
  #[serde(default)]
  #[serde(deserialize_with = "bool_from_int")]
  pub redundant: Option<bool>,
}

/// Used by TCP-based torrent tracker to reply to torrent client.
/// This payload is bencoded and sent in response body (format depends on `compact` parameter in [`Announce`], by default it is a bencoded dictionary).
#[derive(Debug, Serialize, ToSchema)]
pub struct AnnounceReply {
  /// Failure reason.
  /// If [`Some`] then all other fields are hidden.
  #[serde(rename = "failure reason")]
  pub failure_reason: Option<String>,
  /// Mild version of failure reason.
  /// Transmission does not stop in this case, but it serves as notification to user.
  /// This is non-standard.
  #[serde(rename = "warning message")]
  pub warning_message: Option<String>,
  /// When should torrent client send [`Announce`], how frequently?
  pub interval: u64,
  /// What is minimum period torrent client has to wait before sending next [`Announce`]?
  /// This is non-standard.
  #[serde(rename = "min interval")]
  pub min_interval: Option<u64>,
  /// What is tracker id of current tracker.
  #[serde(rename = "tracker id")]
  pub tracker_id: Option<String>,
  /// Number of peers in swarm that have completed download (ie. number of seeds).
  pub complete: u64,
  /// Number of peers in swarm that have NOT completed download (ie. number of leeches).
  pub incomplete: u64,
  /// All peers in swarm stored in [`PeerStream`] which can be either bencoded dict or bencoded string, depending on `compact` parameter in [`Announce`].
  pub peers: PeerStream,
}

impl ToBencode for AnnounceReply {
  const MAX_DEPTH: usize = 10;
  fn encode(&self, encoder: SingleItemEncoder) -> Result<(), encoding::Error> {
    if let Some(ref failure_reason) = self.failure_reason {
      return encoder.emit_dict(|mut d| d.emit_pair(b"failure reason", failure_reason));
    }
    encoder.emit_unsorted_dict(|d| {
      if let Some(ref warning_message) = self.warning_message {
        d.emit_pair(b"warning message", warning_message)?;
      }
      d.emit_pair(b"interval", self.interval)?;
      if let Some(ref min_interval) = self.min_interval {
        d.emit_pair(b"min interval", min_interval)?;
      }
      if let Some(ref tracker_id) = self.tracker_id {
        d.emit_pair(b"tracker id", tracker_id)?;
      }
      d.emit_pair(b"complete", self.complete)?;
      d.emit_pair(b"incomplete", self.incomplete)?;
      d.emit_pair(b"peers", &self.peers)?;
      Ok(())
    })
  }
}
