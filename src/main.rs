#![doc(html_logo_url = "https://sloveniaengineering.github.io/laguna-backend/logo.png")]
#![doc(html_favicon_url = "https://sloveniaengineering.github.io/laguna-backend/favicon.ico")]
#![doc(issue_tracker_base_url = "https://github.com/SloveniaEngineering/laguna-backend")]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

use actix_settings::ApplySettings;

use actix_web::middleware::Logger;
use actix_web::middleware::NormalizePath;
use actix_web::middleware::TrailingSlash;
use actix_web::web;
use actix_web::HttpServer;
use laguna::dto::meta::AppInfoDTO;
use laguna::setup::get_settings;
use std::env;

use laguna::setup::setup;
use laguna::setup::setup_cors;
use laguna::setup::setup_db;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
  HttpServer::new(move || {
    setup()
      .wrap(Logger::default())
      .wrap(setup_cors(&get_settings()))
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
      // FIXME: This shit is so annoying and doesn't work w/FE
      .wrap(NormalizePath::new(TrailingSlash::MergeOnly))
  })
  .apply_settings(&get_settings())
  .run()
  .await
  .expect("Cannot start server");
  Ok(())
}
