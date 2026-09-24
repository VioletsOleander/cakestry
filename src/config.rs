use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    provider: String,
    /// Providers of API service.
    providers: Vec<Provider>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read content from path '{}'", path))?;

        toml::from_str(&content).context("failed to parse config content")
    }

    pub fn find_provider(&self, name: &str) -> Option<&Provider> {
        self.providers
            .iter()
            .find(|&provider| provider.name == name)
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }
}

#[derive(Deserialize, Debug)]
pub struct Provider {
    /// Example: "DeepSeek".
    name: String,
    /// Example: "deepkseek-v4-flash"
    model: String,
    /// Example: "https://api.deepseek.com"
    base_url: String,
    /// Example: "sk-xxx", TODO: use keyring to makes this as secret.
    api_key: String,
}

impl Provider {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}
