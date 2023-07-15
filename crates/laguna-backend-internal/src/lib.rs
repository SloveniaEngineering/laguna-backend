//! This crate is utility.
//! Re-Exports all backend libraries as one library.
#![doc(html_logo_url = "../logo.png")]
#![doc(html_favicon_url = "../favicon.ico")]
#![doc(issue_tracker_base_url = "https://github.com/SloveniaEngineering/laguna-backend")]
pub use laguna_backend_api as api;
pub use laguna_backend_middleware as middleware;
pub use laguna_backend_model as model;
