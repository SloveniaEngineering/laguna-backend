#![doc(html_logo_url = "https://sloveniaengineering.github.io/laguna-backend/logo.png")]
#![doc(html_favicon_url = "https://sloveniaengineering.github.io/laguna-backend/favicon.ico")]
#![doc(issue_tracker_base_url = "https://github.com/SloveniaEngineering/laguna-backend")]
use std::{env, net::SocketAddr};

use actix_settings::BasicSettings;
use const_format::formatcp;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

pub const WORKSPACE_ROOT: &str = env!("WORKSPACE_ROOT");
pub const LAGUNA_CONFIG: &str = formatcp!("{}/Laguna.toml", WORKSPACE_ROOT);
pub const MIGRATIONS_DIR: &str = formatcp!("{}/migrations", WORKSPACE_ROOT);

pub type Settings = BasicSettings<ApplicationSettings>;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ApplicationSettings {
  pub database: DatabaseSettings,
  pub auth: AuthSettings,
  pub frontend: FrontendSettings,
  pub tracker: TrackerSettings,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct AuthSettings {
  pub secret_key: Secret<String>,
  pub password_pepper: Secret<String>,
  pub access_token_lifetime_seconds: i64,
  pub refresh_token_lifetime_seconds: i64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct DatabaseSettings {
  pub proto: String,
  pub host: Secret<String>,
  pub port: u16,
  pub username: String,
  pub password: Secret<String>,
  pub name: String,
}

impl DatabaseSettings {
  pub fn url(&self) -> String {
    format!(
      "{}://{}:{}@{}:{}/{}",
      self.proto,
      self.username,
      self.password.expose_secret(),
      self.host.expose_secret(),
      self.port,
      self.name
    )
  }

  pub fn from_url(url: String) -> Self {
    let url = url::Url::parse(&url).expect("Cannot parse database url");
    Self {
      proto: url.scheme().to_string(),
      host: Secret::new(
        url
          .host_str()
          .expect("Cannot parse database host")
          .to_string(),
      ),
      port: url.port().expect("Cannot parse database port"),
      username: url.username().to_string(),
      password: Secret::new(
        url
          .password()
          .expect("Cannot parse database password")
          .to_string(),
      ),
      name: url.path().trim_start_matches('/').to_string(),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct FrontendSettings {
  pub host: String,
  pub port: u16,
}

impl FrontendSettings {
  pub fn address(&self) -> SocketAddr {
    SocketAddr::new(
      self
        .host
        .parse()
        .expect("Cannot parse frontend host address"),
      self.port,
    )
  }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct TrackerSettings {
  pub announce_url: String,
}

pub fn make_overridable_with_env_vars(settings: &mut Settings) {
  Settings::override_field_with_env_var(&mut settings.actix.hosts, "ACTIX_HOSTS")
    .expect("ACTIX_HOSTS not specified");
  Settings::override_field_with_env_var(&mut settings.actix.mode, "ACTIX_MODE")
    .expect("ACTIX_MODE not specified");
  Settings::override_field_with_env_var(
    &mut settings.actix.enable_compression,
    "ACTIX_ENABLE_COMPRESSION",
  )
  .expect("ACTIX_ENABLE_COMPRESSION not specified");
  Settings::override_field_with_env_var(&mut settings.actix.enable_log, "ACTIX_ENABLE_LOG")
    .expect("ACTIX_ENABLE_LOG not specified");
  Settings::override_field_with_env_var(&mut settings.actix.backlog, "ACTIX_BACKLOG")
    .expect("ACTIX_BACKLOG not specified");
  Settings::override_field_with_env_var(&mut settings.actix.num_workers, "ACTIX_NUM_WORKERS")
    .expect("ACTIX_NUM_WORKERS not specified");
  Settings::override_field_with_env_var(
    &mut settings.actix.max_connections,
    "ACTIX_MAX_CONNECTIONS",
  )
  .expect("ACTIX_MAX_CONNECTIONS not specified");
  Settings::override_field_with_env_var(
    &mut settings.actix.max_connection_rate,
    "ACTIX_MAX_CONNECTION_RATE",
  )
  .expect("ACTIX_MAX_CONNECTION_RATE not specified");
  Settings::override_field_with_env_var(&mut settings.actix.keep_alive, "ACTIX_KEEP_ALIVE")
    .expect("ACTIX_KEEP_ALIVE not specified");
  Settings::override_field_with_env_var(&mut settings.actix.client_timeout, "ACTIX_CLIENT_TIMEOUT")
    .expect("ACTIX_CLIENT_TIMEOUT not specified");
  Settings::override_field_with_env_var(
    &mut settings.actix.client_shutdown,
    "ACTIX_CLIENT_SHUTDOWN",
  )
  .expect("ACTIX_CLIENT_SHUTDOWN not specified");
  Settings::override_field_with_env_var(&mut settings.actix.tls.enabled, "ACTIX_TLS_ENABLED")
    .expect("ACTIX_TLS_ENABLED not specified");
  Settings::override_field_with_env_var(
    &mut settings.actix.tls.certificate,
    "ACTIX_TLS_CERTIFICATE",
  )
  .expect("ACTIX_TLS_CERTIFICATE not specified");
  Settings::override_field_with_env_var(
    &mut settings.actix.tls.private_key,
    "ACTIX_TLS_PRIVATE_KEY",
  )
  .expect("ACTIX_TLS_PRIVATE_KEY not specified");
  if let Ok(application_auth_secret_key) = env::var("APPLICATION_AUTH_SECRET_KEY") {
    settings.application.auth.secret_key = Secret::new(application_auth_secret_key);
  }
  if let Ok(application_auth_password_pepper) = env::var("APPLICATION_AUTH_PASSWORD_PEPPER") {
    settings.application.auth.password_pepper = Secret::new(application_auth_password_pepper);
  }
  Settings::override_field_with_env_var(
    &mut settings.application.auth.access_token_lifetime_seconds,
    "APPLICATION_AUTH_ACCESS_TOKEN_LIFETIME_SECONDS",
  )
  .expect("APPLICATION_ACCESS_AUTH_TOKEN_LIFETIME_SECONDS not specified");
  Settings::override_field_with_env_var(
    &mut settings.application.auth.refresh_token_lifetime_seconds,
    "APPLICATION_AUTH_REFRESH_TOKEN_LIFETIME_SECONDS",
  )
  .expect("APPLICATION_REFRESH_AUTH_TOKEN_LIFETIME_SECONDS not specified");
  Settings::override_field_with_env_var(
    &mut settings.application.database.proto,
    "APPLICATION_DATABASE_PROTO",
  )
  .expect("APPLICATION_DATABASE_PROTO not specified");
  if let Ok(application_database_host) = env::var("APPLICATION_DATABASE_HOST") {
    settings.application.database.host = Secret::new(application_database_host);
  }
  Settings::override_field_with_env_var(
    &mut settings.application.database.port,
    "APPLICATION_DATABASE_PORT",
  )
  .expect("APPLICATION_DATABASE_PORT not specified");
  Settings::override_field_with_env_var(
    &mut settings.application.database.username,
    "APPLICATION_DATABASE_USERNAME",
  )
  .expect("APPLICATION_DATABASE_USERNAME not specified");
  if let Ok(application_database_password) = env::var("APPLICATION_DATABASE_PASSWORD") {
    settings.application.database.password = Secret::new(application_database_password);
  }
  Settings::override_field_with_env_var(
    &mut settings.application.database.name,
    "APPLICATION_DATABASE_NAME",
  )
  .expect("APPLICATION_DATABASE_NAME not specified");
  Settings::override_field_with_env_var(
    &mut settings.application.frontend.host,
    "APPLICATION_FRONTEND_HOST",
  )
  .expect("APPLICATION_FRONTEND_HOST not specified");
  Settings::override_field_with_env_var(
    &mut settings.application.frontend.port,
    "APPLICATION_FRONTEND_PORT",
  )
  .expect("APPLICATION_FRONTEND_PORT not specified");
  if let Ok(database_url) = env::var("DATABASE_URL") {
    settings.application.database = DatabaseSettings::from_url(database_url);
  }
  Settings::override_field_with_env_var(
    &mut settings.application.tracker.announce_url,
    "APPLICATION_TRACKER_ANNOUNCE_URL",
  )
  .expect("APPLICATION_TRACKER_ANNOUNCE_URL not specified");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_database_url_from_url() {
    let db_settings = DatabaseSettings::from_url(String::from(
      "postgres://postgres:postgres@127.0.0.1:5432/laguna_dev_db",
    ));
    assert_eq!(db_settings.proto, String::from("postgres"));
    assert_eq!(
      db_settings.host.expose_secret().to_string(),
      String::from("127.0.0.1")
    );
    assert_eq!(db_settings.port, 5432);
    assert_eq!(db_settings.username, String::from("postgres"));
    assert_eq!(
      db_settings.password.expose_secret().to_string(),
      String::from("postgres")
    );
    assert_eq!(db_settings.name, String::from("laguna_dev_db"));
  }
}
