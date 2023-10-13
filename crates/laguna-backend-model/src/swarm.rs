use std::collections::HashSet;

use crate::peer::Peer;

// should be hashset but sqlx doesn't support it
pub type Swarm = HashSet<Peer>;

/*
pub struct Swarm {
    pub peers: HashSet<Peer>,
}
*/
