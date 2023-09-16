use std::net::IpAddr;

use laguna_backend_tracker_common::{
  announce::{AnnounceEvent, Announcement, AnnouncementResponse},
  info_hash::InfoHash,
  peer::{PeerId, PeerStream},
};

use serde::{Deserialize, Serialize};

use laguna_backend_tracker_common::helpers::bool_from_int;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Announce<const N: usize> {
  pub info_hash: InfoHash<N>,
  pub peer_id: PeerId,
  pub ip: Option<IpAddr>,
  pub port: u16,
  pub uploaded: i64,
  pub downloaded: i64,
  pub left: i64,
  pub event: Option<AnnounceEvent>,
  pub numwant: Option<i64>,
  #[serde(default)]
  #[serde(deserialize_with = "bool_from_int")]
  pub compact: Option<bool>,
  #[serde(default)]
  #[serde(deserialize_with = "bool_from_int")]
  pub no_peer_id: Option<bool>,
  pub key: Option<String>,
  pub trackerid: Option<String>,
  #[serde(default)]
  #[serde(deserialize_with = "bool_from_int")]
  pub supportcrypto: Option<bool>,
  #[serde(default)]
  #[serde(deserialize_with = "bool_from_int")]
  pub redundant: Option<bool>,
}

impl<const N: usize> Announcement<N> for Announce<N> {
  #[inline]
  fn peer_id(&self) -> &PeerId {
    &self.peer_id
  }

  #[inline]
  fn info_hash(&self) -> &InfoHash<N> {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct AnnounceReply {
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

impl ToBencode for AnnounceReply {
  const MAX_DEPTH: usize = 10;
  fn encode(&self, encoder: SingleItemEncoder) -> Result<(), encoding::Error> {
    if let Some(ref failure_reason) = self.failure_reason {
      return encoder.emit_dict(|mut d| d.emit_pair(b"failure reason", failure_reason));
    }
    encoder.emit_unsorted_dict(|d| {
      d.emit_pair(
        b"failure reason",
        self.failure_reason.clone().unwrap_or_default(),
      )?;
      d.emit_pair(
        b"warning message",
        &self.warning_message.clone().unwrap_or_default(),
      )?;
      d.emit_pair(b"interval", self.interval)?;
      d.emit_pair(b"min interval", self.min_interval.unwrap_or(self.interval))?;
      d.emit_pair(b"tracker id", &self.tracker_id.clone().unwrap_or_default())?;
      d.emit_pair(b"complete", self.complete)?;
      d.emit_pair(b"incomplete", self.incomplete)?;
      d.emit_pair(b"peers", &self.peers)?;
      Ok(())
    })
  }
}
