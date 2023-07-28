use std::path::PathBuf;

use anyhow::Result;
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use serde::Deserialize;
use time::OffsetDateTime;
use url::Url;

pub static CONFIG: Lazy<Config> =
    Lazy::new(|| Config::try_from_env().expect("failed to parse config from env vars"));

fn default_listen_addr() -> String {
    "0.0.0.0:3000".to_string()
}

fn default_database_host() -> String {
    "localhost".to_string()
}

fn default_database_port() -> u16 {
    5432
}

fn default_database_user() -> String {
    "postgres".to_string()
}

fn default_database_password() -> String {
    "contcont".to_string()
}

fn default_database_database() -> String {
    "postgres".to_string()
}

fn default_static_files_directory_path() -> PathBuf {
    PathBuf::from("../frontend/dist")
}

fn default_literature_enabled() -> bool {
    true
}

fn default_art_enabled() -> bool {
    true
}

fn deserialize_jwt_secret<'de, D>(d: D) -> Result<(EncodingKey, DecodingKey), D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = std::borrow::Cow::<'_, str>::deserialize(d)?;
    Ok((
        EncodingKey::from_secret(s.as_bytes()),
        DecodingKey::from_secret(s.as_bytes()),
    ))
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub contest_name: String,

    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,
    pub base_url: Url,

    #[serde(default = "default_database_host")]
    pub database_host: String,
    #[serde(default = "default_database_port")]
    pub database_port: u16,
    #[serde(default = "default_database_user")]
    pub database_user: String,
    #[serde(default = "default_database_password")]
    pub database_password: String,
    #[serde(default = "default_database_database")]
    pub database_database: String,

    #[serde(default = "default_static_files_directory_path")]
    pub static_files_directory_path: PathBuf,

    #[serde(deserialize_with = "deserialize_jwt_secret")]
    pub jwt_secret: (EncodingKey, DecodingKey),

    #[serde(default = "default_literature_enabled")]
    pub literature_enabled: bool,
    #[serde(default = "default_art_enabled")]
    pub art_enabled: bool,

    #[serde(with = "time::serde::rfc3339")]
    pub submission_open_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub submission_close_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub voting_open_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub voting_close_at: OffsetDateTime,
}

impl Config {
    pub fn try_from_env() -> Result<Self> {
        let config: Config = envy::from_env()?;
        Ok(config)
    }
}
