use std::collections::HashSet;

use crate::peer::Peer;

/// Convenient type for torrent's swarm.
// TODO: should be hashset but sqlx doesn't support it
pub type Swarm = HashSet<Peer>;
