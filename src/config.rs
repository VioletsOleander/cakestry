use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    default_provider: String,
    /// Providers of API service.
    providers: Vec<Provider>,
}

impl Config {
    /// Return a [`Config`] instance constructed from the content of `f`.
    ///
    /// `f` is expected to be a relative path to the configuration file.
    pub fn from_file(f: &str) -> Config {
        let content = fs::read_to_string(f).unwrap_or_else(|e| {
            panic!(
                "A configuration file should exist in given path {}: {}",
                f, e
            )
        });

        toml::from_str(&content).expect("The format of config.toml should be valid to parse")
    }
}

#[derive(Deserialize, Debug)]
struct Provider {
    /// Example: "DeepSeek".
    name: String,
    /// Example: "deepkseek-v4-flash"
    model: String,
    /// Example: "https://api.deepseek.com"
    base_url: String,
    /// Example: "sk-xxx", TODO: use keyring to makes this as secret.
    api_key: String,
}
