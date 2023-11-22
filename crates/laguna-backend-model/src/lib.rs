#![doc(html_logo_url = "https://sloveniaengineering.github.io/laguna-backend/logo.png")]
#![doc(html_favicon_url = "https://sloveniaengineering.github.io/laguna-backend/favicon.ico")]
#![doc(issue_tracker_base_url = "https://github.com/SloveniaEngineering/laguna-backend")]
#![deny(missing_docs)]
//! Models, and anything DB.

/// [`Peer`] and [`User`] behaviour related structures.
pub mod behaviour;
/// Constants (used by validators).
pub mod consts;
/// Download model.
pub mod download;
/// Genre enum.
pub mod genre;
/// Peer model.
pub mod peer;
/// Rating model.
pub mod rating;
/// Role enum.
pub mod role;
/// Speedlevel enum and speedlevel calculation logic.
pub mod speedlevel;
/// Swarm structures.
pub mod swarm;
/// Torrent model.
pub mod torrent;
/// Torrent rating model.
pub mod torrent_rating;
/// User model.
pub mod user;
/// View-based models.
pub mod views;
