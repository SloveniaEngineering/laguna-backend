use std::net::IpAddr;

use laguna_backend_tracker_common::{
  announce::{AnnounceEvent, Announcement, AnnouncementResponse},
  info_hash::{InfoHash, SHA1_LENGTH},
  peer::{PeerId, PeerStream},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnounceRequest {
  pub info_hash: InfoHash<SHA1_LENGTH>,
  pub peer_id: PeerId,
  pub ip: Option<IpAddr>,
  pub port: u16,
  pub uploaded: i64,
  pub downloaded: i64,
  pub left: i64,
  pub event: Option<AnnounceEvent>,
  pub numwant: Option<i64>,
  pub compact: Option<bool>,
  pub no_peer_id: Option<bool>,
  pub key: Option<String>,
  pub trackerid: Option<String>,
}

impl Announcement for AnnounceRequest {
  #[inline]
  fn peer_id(&self) -> &PeerId {
    &self.peer_id
  }

  #[inline]
  fn info_hash(&self) -> &InfoHash<SHA1_LENGTH> {
    &self.info_hash
  }

  #[inline]
  fn uploaded(&self) -> i64 {
    self.uploaded
  }

  #[inline]
  fn downloaded(&self) -> i64 {
    self.downloaded
  }

  #[inline]
  fn left(&self) -> i64 {
    self.left
  }

  #[inline]
  fn event(&self) -> Option<AnnounceEvent> {
    self.event
  }

  #[inline]
  fn no_peer_id(&self) -> bool {
    self.no_peer_id.unwrap_or(false)
  }

  #[inline]
  fn port(&self) -> u16 {
    self.port
  }

  #[inline]
  fn ip(&self) -> Option<IpAddr> {
    self.ip
  }

  #[inline]
  fn numwant(&self) -> i64 {
    self.numwant.unwrap_or(50)
  }

  #[inline]
  fn key(&self) -> Option<&String> {
    self.key.as_ref()
  }

  #[inline]
  fn trackerid(&self) -> Option<&String> {
    self.trackerid.as_ref()
  }

  #[inline]
  fn compact(&self) -> bool {
    self.compact.unwrap_or(false)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnounceResponse {
  #[serde(rename = "failure reason")]
  pub failure_reason: Option<String>,
  #[serde(rename = "warning message")]
  pub warning_message: Option<String>,
  pub interval: u64,
  #[serde(rename = "min interval")]
  pub min_interval: Option<u64>,
  #[serde(rename = "tracker id")]
  pub tracker_id: Option<String>,
  pub complete: u64,
  pub incomplete: u64,
  pub peers: PeerStream,
}

impl AnnouncementResponse for AnnounceResponse {
  #[inline]
  fn failure_reason(&self) -> Option<&String> {
    self.failure_reason.as_ref()
  }

  #[inline]
  fn warning_message(&self) -> Option<&String> {
    self.warning_message.as_ref()
  }

  #[inline]
  fn interval(&self) -> u64 {
    self.interval
  }

  #[inline]
  fn min_interval(&self) -> Option<u64> {
    self.min_interval
  }

  #[inline]
  fn tracker_id(&self) -> Option<&String> {
    self.tracker_id.as_ref()
  }

  #[inline]
  fn complete(&self) -> u64 {
    self.complete
  }

  #[inline]
  fn incomplete(&self) -> u64 {
    self.incomplete
  }

  #[inline]
  fn peers(&self) -> &PeerStream {
    &self.peers
  }
}
