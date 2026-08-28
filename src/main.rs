use clap::Parser;
use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;

use cakestry::App;

/// AI agent in the command line
#[derive(Parser)]
struct Args {}

fn main() {
    // Initialize the default global tracing subscriber.
    let appender = rolling::never(".", "cakestry.log");
    let subscriber_builder = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("CAKESTRY_LOG"))
        .with_ansi(false)
        .with_writer(appender);

    subscriber_builder.init();

    let _ = Args::parse();
    let mut app = App::default();

    app.run();
}
