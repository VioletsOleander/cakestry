use clap::Parser;
use clap::builder::Styles;

#[derive(Parser)]
#[command(version, about)]
#[command(styles = Styles::default())]
pub struct Args {
    /// Relative path to the config file.
    #[arg(long, default_value = ".cakestry/config.toml")]
    config_path: String,
    /// Relative path to the log file.
    #[arg(long, default_value = "cakestry.log")]
    log_path: String,
}

impl Args {
    pub fn config_path(&self) -> &str {
        &self.config_path
    }

    pub fn log_path(&self) -> &str {
        &self.log_path
    }
}
