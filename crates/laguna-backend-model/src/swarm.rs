use crate::peer::Peer;

// should be hashset but sqlx doesn't support it
pub type Swarm = Vec<Peer>;
