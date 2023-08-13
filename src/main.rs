#![doc(html_logo_url = "https://sloveniaengineering.github.io/laguna-backend/logo.png")]
#![doc(html_favicon_url = "https://sloveniaengineering.github.io/laguna-backend/favicon.ico")]
#![doc(issue_tracker_base_url = "https://github.com/SloveniaEngineering/laguna-backend")]
#![doc = include_str!("../README.md")]

use actix_settings::ApplySettings;

use actix_web::middleware::Logger;
use actix_web::HttpServer;
use laguna::setup::get_settings;

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
  })
  .apply_settings(&get_settings())
  .run()
  .await
  .expect("Cannot start server");
  Ok(())
}
