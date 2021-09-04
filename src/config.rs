use serde::{Deserialize, Serialize};
use std::fs;
use std::net::Ipv4Addr;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    ParseError(#[from] serde_json::Error),

    #[error(transparent)]
    ReadError(#[from] std::io::Error),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub devices: Devices,
    pub actions: Vec<Actions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Devices {
    pub keylight: Vec<String>,
    pub motu: Ipv4Addr,
    pub camera: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Actions {
    pub btn: u8,
    pub module: String,
    pub action: String,
    pub desc: String,
    pub value: Option<bool>,
}

impl Config {
    pub fn load_config(path: &str) -> Result<Arc<Config>, ConfigError> {
        let data = fs::read_to_string(path)?;
        let cfg: Config = serde_json::from_str(&data)?;
        Ok(Arc::new(cfg))
    }
}
