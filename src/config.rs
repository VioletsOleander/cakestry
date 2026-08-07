use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    default_provider: String,
    /// Providers of API service.
    providers: Vec<Provider>,
}

impl Config {
    /// Return a [`Config`] instance constructed from the content of `file`.
    ///
    /// `file` is expected to be a relative path to the configuration file.
    pub fn from_file(file: &str) -> Config {
        let content = fs::read_to_string(file).unwrap_or_else(|e| {
            panic!(
                "A configuration file should exist in given path {}: {}",
                file, e
            )
        });

        toml::from_str(&content).expect("The format of config.toml should be valid to parse")
    }

    /// Search for a provider by the given `name`, if found, return it.
    pub fn get_provider(&self, name: &str) -> Option<&Provider> {
        self.providers
            .iter()
            .find(|&provider| provider.name == name)
    }

    pub fn default_provider(&self) -> &str {
        &self.default_provider
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
