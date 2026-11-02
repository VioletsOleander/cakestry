use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub system_prompt: String,
    pub provider: String,
    pub providers: Vec<Provider>,
    pub tools: Option<Vec<Tool>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Provider {
    pub name: String,
    pub model: String,
    pub base_url: String,
    // TODO: use keyring to makes this as secret.
    pub api_key: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Tool {
    Function {
        name: String,
        parameters: Option<String>,
        description: Option<String>,
    },
    WebSearch,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read content from path '{}'", path))?;

        toml::from_str(&content).context("failed to parse config content")
    }
}
