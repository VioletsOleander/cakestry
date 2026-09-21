use anyhow::Result;
use clap::Parser;

use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;

mod arg;
mod config;

use arg::CakestryArgs;
use config::Config;

fn main() -> Result<()> {
    let args = CakestryArgs::parse();

    let config = Config::from_file(args.config_path())?;

    init_subscriber(args.log_path());
    // let mut app = App::default();
    //
    // app.run();

    Ok(())
}

/// Initialize the default global tracing subscriber.
fn init_subscriber(log_path: &str) {
    let appender = rolling::never(".", log_path);
    let subscriber_builder = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("CAKESTRY_LOG"))
        .with_ansi(false)
        .with_writer(appender);

    subscriber_builder.init();
}
