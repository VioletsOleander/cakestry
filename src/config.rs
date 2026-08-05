use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    default_provider: String,
    /// Providers of API service.
    providers: Vec<Provider>,
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

/// Return a [`Config`] instance constructed from the content of file `.cakestry/config.toml`.
pub fn load_config() -> Config {
    let content = fs::read_to_string(".cakestry/config.toml")
        .expect("A config.toml file should exists under the .cakestry/ directory.");

    toml::from_str(&content).expect("The format of config.toml should be valid to parse.")
}
