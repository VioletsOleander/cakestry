use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub system_prompt: String,
    pub default_provider: String,
    pub providers: Vec<Provider>,
    pub tools: Option<Vec<Tool>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Provider {
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    pub models: Vec<String>,
    pub default_model: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tool {
    Function {
        name: String,
        parameters: Option<String>,
        description: Option<String>,
        enabled: bool,
    },
    WebSearch {
        name: String,
        enabled: bool,
    },
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read content from path '{}'", path))?;

        toml::from_str(&content).context("failed to parse config content")
    }
}
