use std::net::SocketAddr;

use actix_settings::BasicSettings;
use const_format::formatcp;
use serde::{Deserialize, Serialize};

pub const WORKSPACE_ROOT: &str = env!("WORKSPACE_ROOT");

pub const CONFIG_DIR: &str = formatcp!("{}/configs", WORKSPACE_ROOT);
pub const MIGRATIONS_DIR: &str = formatcp!("{}/migrations", WORKSPACE_ROOT);

pub const CONFIG_PROD_NAME: &str = "prod.toml";
pub const CONFIG_DEV_NAME: &str = "dev.toml";
pub const CONFIG_DEV: &str = formatcp!("{}/{}", CONFIG_DIR, CONFIG_DEV_NAME);
pub const CONFIG_PROD: &str = formatcp!("{}/{}", CONFIG_DIR, CONFIG_PROD_NAME);

pub type Settings = BasicSettings<ApplicationSettings>;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ApplicationSettings {
    pub database: DatabaseSettings,
    pub auth: AuthSettings,
    pub frontend: FrontendSettings,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AuthSettings {
    pub secret_key: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DatabaseSettings {
    pub proto: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
}

impl DatabaseSettings {
    pub fn url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.proto, self.username, self.password, self.host, self.port, self.name
        )
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