use std::net::IpAddr;

use crate::{
    info_hash::InfoHash,
    peer::{PeerId, PeerStream},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnounceEvent {
    Started,
    Stopped,
    Completed,
}

/// Announcement trait shared by HTTP, UDP and WS Announcement Messages.
pub trait Announcement {
    fn peer_id(&self) -> &PeerId;
    fn info_hash(&self) -> &InfoHash;
    fn uploaded(&self) -> u64;
    fn downloaded(&self) -> u64;
    fn left(&self) -> u64;
    fn event(&self) -> Option<AnnounceEvent> {
        None
    }
    fn no_peer_id(&self) -> bool {
        false
    }
    fn port(&self) -> u16;
    fn ip(&self) -> Option<IpAddr> {
        None
    }
    fn numwant(&self) -> u64 {
        50
    }
    fn key(&self) -> Option<&String> {
        None
    }
    fn trackerid(&self) -> Option<&String> {
        None
    }
    fn compact(&self) -> bool {
        false
    }
}

/// Announcement response trait shared by HTTP, UDP and WS Announcement Response Messages.
pub trait AnnouncementResponse {
    fn failure_reason(&self) -> Option<&String>;
    fn warning_message(&self) -> Option<&String> {
        None
    }
    fn interval(&self) -> u64;
    fn min_interval(&self) -> Option<u64> {
        None
    }
    fn tracker_id(&self) -> Option<&String> {
        None
    }
    fn complete(&self) -> u64;
    fn incomplete(&self) -> u64;
    fn peers(&self) -> &PeerStream;
}
