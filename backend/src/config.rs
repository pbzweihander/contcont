use std::path::PathBuf;

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::Deserialize;

pub static CONFIG: Lazy<Config> =
    Lazy::new(|| Config::try_from_env().expect("failed to parse config from env vars"));

fn default_listen_addr() -> String {
    "0.0.0.0:3000".to_string()
}

fn default_static_files_directory() -> PathBuf {
    PathBuf::from("../frontend/dist")
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,

    #[serde(default = "default_static_files_directory")]
    pub static_files_directory: PathBuf,
}

impl Config {
    pub fn try_from_env() -> Result<Self> {
        let config: Config = envy::from_env()?;
        Ok(config)
    }
}
