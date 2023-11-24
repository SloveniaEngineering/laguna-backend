#![doc(html_logo_url = "https://sloveniaengineering.github.io/laguna-backend/logo.svg")]
#![doc(html_favicon_url = "https://sloveniaengineering.github.io/laguna-backend/favicon.ico")]
#![doc(issue_tracker_base_url = "https://github.com/SloveniaEngineering/laguna-backend")]
#![deny(missing_docs)]

//! Defines a set of structures such as [`PeerId`], [`InfoHash`], and so on, used by all TCP-based, UDP-based and WS-based trackers.
/// Announce-related structures
pub mod announce;
/// Deserialization and serialization helpers.
pub mod helpers;
/// Info hash structure.
pub mod info_hash;
/// Peer structures.
pub mod peer;
