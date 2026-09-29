use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub system_prompt: String,
    pub provider: String,
    pub providers: Vec<Provider>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read content from path '{}'", path))?;

        toml::from_str(&content).context("failed to parse config content")
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Provider {
    /// Example: "DeepSeek".
    pub name: String,
    /// Example: "deepkseek-v4-flash"
    pub model: String,
    /// Example: "https://api.deepseek.com"
    pub base_url: String,
    /// Example: "sk-xxx", TODO: use keyring to makes this as secret.
    pub api_key: String,
}
