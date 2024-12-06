use std::fs;

use dioxus::prelude::ServerFnError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub chat: ChatConfig,
}

#[derive(Debug, Deserialize)]
pub struct ChatConfig {
    pub component_wasm_path: String,
    pub component_handler: String,
    pub component_handle_function: String,
    pub provider: String,
    pub api_key: String,
    pub model: String,
}

pub fn get_config() -> Result<Config, ServerFnError> {
    let contents = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
