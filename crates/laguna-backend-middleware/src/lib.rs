#![doc(html_logo_url = "https://sloveniaengineering.github.io/laguna-backend/logo.svg")]
#![doc(html_favicon_url = "https://sloveniaengineering.github.io/laguna-backend/favicon.ico")]
#![doc(issue_tracker_base_url = "https://github.com/SloveniaEngineering/laguna-backend")]
#![deny(missing_docs)]
//! Middleware logic.
#[allow(missing_docs)]
pub mod auth;
#[allow(missing_docs)]
pub mod consts;
/// URL hexification middleware.
/// Used to avoid escaping NON-Url strings into Url strings in particularly various hashes.
pub mod hexify;
/// Custom MIMEs.
pub mod mime;
