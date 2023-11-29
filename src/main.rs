#![doc(html_logo_url = "https://sloveniaengineering.github.io/laguna-backend/logo.svg")]
#![doc(html_favicon_url = "https://sloveniaengineering.github.io/laguna-backend/favicon.ico")]
#![doc(issue_tracker_base_url = "https://github.com/SloveniaEngineering/laguna-backend")]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]

use actix_settings::ApplySettings;

use actix_web::middleware::Logger;
use actix_web::middleware::NormalizePath;
use actix_web::middleware::TrailingSlash;
use actix_web::web;
use actix_web::HttpServer;

use laguna::config::get_settings;
use laguna::dto::meta::AppInfoDTO;
use std::env;
use actix_governor::Governor;

use laguna::setup::setup_cors;
use laguna::setup::setup_db;
use laguna::setup::{setup, setup_ip_ratelimiter};

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
  HttpServer::new(move || {
    setup()
      .data_factory(|| async move { setup_db(&get_settings()).await })
      .app_data(web::Data::new(AppInfoDTO {
        version: env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION not set"),
        authors: env::var("CARGO_PKG_AUTHORS")
          .expect("CARGO_PKG_AUTHORS not set")
          .split(':')
          .map(ToString::to_string)
          .collect::<Vec<String>>(),
        license: env::var("CARGO_PKG_LICENSE").expect("CARGO_PKG_LICENSE not set"),
        description: env::var("CARGO_PKG_DESCRIPTION").expect("CARGO_PKG_DESCRIPTION not set"),
        repository: env::var("CARGO_PKG_REPOSITORY").expect("CARGO_PKG_REPOSITORY not set"),
      }))
      .wrap(Governor::new(&setup_ip_ratelimiter!()))
      .wrap(setup_cors(&get_settings()))
      .wrap(NormalizePath::new(TrailingSlash::MergeOnly))
      .wrap(Logger::default())
  })
  .apply_settings(&get_settings())
  .run()
  .await
  .expect("Cannot start server");
  Ok(())
}
