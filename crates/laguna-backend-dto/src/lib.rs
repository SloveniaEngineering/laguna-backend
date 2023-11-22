#![doc(html_logo_url = "https://sloveniaengineering.github.io/laguna-backend/logo.png")]
#![doc(html_favicon_url = "https://sloveniaengineering.github.io/laguna-backend/favicon.ico")]
#![doc(issue_tracker_base_url = "https://github.com/SloveniaEngineering/laguna-backend")]
#![deny(missing_docs)]
//! DTOs (Data-transfer objects) for communication between BE and FE.
//! Home to validation and some sanitization.
/// Structures relating to "already exists" type errors.
pub mod already_exists;
/// Login-related structures.
pub mod login;
/// Application metadata related structures.
pub mod meta;
/// Peer DTO related structures.
pub mod peer;
/// Rating DTO related structures.
pub mod rating;
/// Registration DTO.
pub mod register;
/// Role DTO.
pub mod role;
/// Torrent DTO.
pub mod torrent;
/// Torrent rating DTO.
pub mod torrent_rating;
/// User DTO.
pub mod user;
/// Custom validators.
pub mod validators;
