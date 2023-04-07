use anyhow::Result;
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Deserialize, Serialize)]
pub struct ListenerConfig {
    pub address: String,
    pub timeout: u32,
}

impl Default for ListenerConfig {
    fn default() -> Self {
        Self {
            // Default listener address
            address: "127.0.0.1:8080".to_string(),
            timeout: 300_000,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub listeners: Vec<ListenerConfig>,
    pub storage_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            // Default of one listener
            listeners: vec![ListenerConfig::default()],
            // Default storage dir
            storage_path: "storage".to_string(),
        }
    }
}

impl Config {
    pub fn parse(path: Option<String>) -> Result<Self> {
        let mut config = Figment::new().merge(Serialized::defaults(Config::default()));
        config = config.merge(Serialized::defaults(ListenerConfig::default()));
        if let Some(path) = path {
            config = config.merge(Toml::file(path));
        }
        Ok(config.extract()?)
    }
}
