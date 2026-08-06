use anyhow::Result;
use clap::Parser;

use cakestry::App;

/// AI agent in the command line
#[derive(Parser)]
struct Args {}

fn main() -> Result<()> {
    // Initialize the default global tracing subscriber.
    let appender = tracing_appender::rolling::never(".", "cakestry.log");
    let subscriber_builder = tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(appender);

    subscriber_builder.init();

    let _ = Args::parse();
    let mut app = App::default();

    app.run()
}
