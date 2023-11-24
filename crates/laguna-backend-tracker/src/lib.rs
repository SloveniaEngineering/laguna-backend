#![doc(html_logo_url = "https://sloveniaengineering.github.io/laguna-backend/logo.svg")]
#![doc(html_favicon_url = "https://sloveniaengineering.github.io/laguna-backend/favicon.ico")]
#![doc(issue_tracker_base_url = "https://github.com/SloveniaEngineering/laguna-backend")]
#![forbid(missing_docs)]
//! Crate abstraction for easy export in internal level crate.

#[cfg(feature = "http")]
pub use laguna_backend_tracker_http as http;
#[cfg(feature = "udp")]
pub use laguna_backend_tracker_udp as udp;
#[cfg(feature = "ws")]
pub use laguna_backend_tracker_ws as ws;

pub use laguna_backend_tracker_common as prelude;
