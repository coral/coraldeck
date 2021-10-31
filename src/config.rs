use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub streamdeck: StreamdeckConfig,
    pub modules: BTreeMap<String, toml::Value>,
    pub action: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
#[serde(rename_all = "camelCase")]
pub struct StreamdeckConfig {
    pub brightness: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub btn: u8,
    pub module: String,
    pub action: String,
    pub desc: String,
    pub value: String,
    pub display: Option<String>,
}

impl Config {
    pub fn load_config(path: &str) -> Result<Arc<Config>, Error> {
        let data = fs::read_to_string(path)?;
        let cfg: Config = toml::from_str(&data)?;
        Ok(Arc::new(cfg))
    }
}
