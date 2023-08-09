use std::{net::SocketAddr, env};

use actix_settings::BasicSettings;
use const_format::formatcp;
use serde::{Deserialize, Serialize};
use secrecy::{Secret, ExposeSecret};

pub const WORKSPACE_ROOT: &str = env!("WORKSPACE_ROOT");

pub const CONFIG_DIR: &str = formatcp!("{}/configs", WORKSPACE_ROOT);
pub const MIGRATIONS_DIR: &str = formatcp!("{}/migrations", WORKSPACE_ROOT);

pub const CONFIG_PROD_NAME: &str = "prod.toml";
pub const CONFIG_DEV_NAME: &str = "dev.toml";
pub const CONFIG_DEV: &str = formatcp!("{}/{}", CONFIG_DIR, CONFIG_DEV_NAME);
pub const CONFIG_PROD: &str = formatcp!("{}/{}", CONFIG_DIR, CONFIG_PROD_NAME);

pub type Settings = BasicSettings<ApplicationSettings>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ApplicationSettings {
    pub database: DatabaseSettings,
    pub auth: AuthSettings,
    pub frontend: FrontendSettings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AuthSettings {
    pub secret_key: Secret<String>,
    pub access_token_lifetime_seconds: i64,
    pub refresh_token_lifetime_seconds: i64,
}

#[derive(Debug, Deserialize)]
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
            self.proto, self.username, self.password.expose_secret(), self.host.expose_secret(), self.port, self.name
        )
    }

    pub fn from_url(url: String) -> Self {
        let url = url::Url::parse(&url).expect("Cannot parse database url");
        Self {
            proto: url.scheme().to_string(),
            host: Secret::new(url.host_str().expect("Cannot parse database host").to_string()),
            port: url.port().expect("Cannot parse database port"),
            username: url.username().to_string(),
            password: Secret::new(url.password().expect("Cannot parse database password").to_string()),
            name: url.path().trim_start_matches('/').to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FrontendSettings {
    pub host: String,
    pub port: u16 
}

impl FrontendSettings {
    pub fn address(&self) -> SocketAddr {
        SocketAddr::new(self.host.parse().expect("Cannot parse frontend host address"), self.port)
    }
}

pub fn make_overridable_with_env_vars(settings: &mut Settings) {
    Settings::override_field_with_env_var(&mut settings.actix.hosts, "ACTIX_HOSTS").expect("ACTIX_HOSTS not specified");
    Settings::override_field_with_env_var(&mut settings.actix.mode, "ACTIX_MODE").expect("ACTIX_MODE not specified");
    Settings::override_field_with_env_var(&mut settings.actix.enable_compression, "ACTIX_ENABLE_COMPRESSION").expect("ACTIX_ENABLE_COMPRESSION not specified");
    Settings::override_field_with_env_var(&mut settings.actix.enable_log, "ACTIX_ENABLE_LOG").expect("ACTIX_ENABLE_LOG not specified");
    Settings::override_field_with_env_var(&mut settings.actix.backlog, "ACTIX_BACKLOG").expect("ACTIX_BACKLOG not specified");
    Settings::override_field_with_env_var(&mut settings.actix.num_workers, "ACTIX_NUM_WORKERS").expect("ACTIX_NUM_WORKERS not specified");
    Settings::override_field_with_env_var(&mut settings.actix.max_connections, "ACTIX_MAX_CONNECTIONS").expect("ACTIX_MAX_CONNECTIONS not specified");
    Settings::override_field_with_env_var(&mut settings.actix.max_connection_rate, "ACTIX_MAX_CONNECTION_RATE").expect("ACTIX_MAX_CONNECTION_RATE not specified");
    Settings::override_field_with_env_var(&mut settings.actix.keep_alive, "ACTIX_KEEP_ALIVE").expect("ACTIX_KEEP_ALIVE not specified");
    Settings::override_field_with_env_var(&mut settings.actix.client_timeout, "ACTIX_CLIENT_TIMEOUT").expect("ACTIX_CLIENT_TIMEOUT not specified");
    Settings::override_field_with_env_var(&mut settings.actix.client_shutdown, "ACTIX_CLIENT_SHUTDOWN").expect("ACTIX_CLIENT_SHUTDOWN not specified");
    Settings::override_field_with_env_var(&mut settings.actix.tls.enabled, "ACTIX_TLS_ENABLED").expect("ACTIX_TLS_ENABLED not specified");
    Settings::override_field_with_env_var(&mut settings.actix.tls.certificate, "ACTIX_TLS_CERTIFICATE").expect("ACTIX_TLS_CERTIFICATE not specified");
    Settings::override_field_with_env_var(&mut settings.actix.tls.private_key, "ACTIX_TLS_PRIVATE_KEY").expect("ACTIX_TLS_PRIVATE_KEY not specified");
    if let Ok(application_secret_key) = env::var("APPLICATION_SECRET_KEY") {
        settings.application.auth.secret_key = Secret::new(application_secret_key);
    }
    Settings::override_field_with_env_var(&mut settings.application.auth.access_token_lifetime_seconds, "APPLICATION_ACCESS_TOKEN_LIFETIME_SECONDS").expect("ACCESS_TOKEN_LIFETIME_SECONDS not specified");
    Settings::override_field_with_env_var(&mut settings.application.auth.refresh_token_lifetime_seconds, "APPLICATION_REFRESH_TOKEN_LIFETIME_SECONDS").expect("REFRESH_TOKEN_LIFETIME_SECONDS not specified");
    Settings::override_field_with_env_var(&mut settings.application.database.proto, "APPLICATION_DATABASE_PROTO").expect("DATABASE_PROTO not specified");
    if let Ok(application_database_host) = env::var("APPLICATION_DATABASE_HOST") {
        settings.application.database.host = Secret::new(application_database_host);
    }
    Settings::override_field_with_env_var(&mut settings.application.database.port, "APPLICATION_DATABASE_PORT").expect("DATABASE_PORT not specified");
    Settings::override_field_with_env_var(&mut settings.application.database.username, "APPLICATION_DATABASE_USERNAME").expect("DATABASE_USERNAME not specified");
    if let Ok(application_database_password) = env::var("APPLICATION_DATABASE_PASSWORD") {
        settings.application.database.password = Secret::new(application_database_password);
    }
    Settings::override_field_with_env_var(&mut settings.application.database.name, "APPLICATION_DATABASE_NAME").expect("DATABASE_NAME not specified");
    Settings::override_field_with_env_var(&mut settings.application.frontend.host, "APPLICATION_FRONTEND_HOST").expect("FRONTEND_HOST not specified");
    Settings::override_field_with_env_var(&mut settings.application.frontend.port, "APPLICATION_FRONTEND_PORT").expect("FRONTEND_PORT not specified");
    if let Ok(database_url) = env::var("DATABASE_URL") {
        settings.application.database = DatabaseSettings::from_url(database_url);
    }
}