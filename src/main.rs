use anyhow::Result;
use clap::Parser;

use crossbeam_channel::{Select, bounded};
use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;

mod arg;
mod config;
mod event;
mod service;
mod terminal;

use arg::Args;
use config::Config;
use event::TerminalEventListener;

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::from_file(args.config_path())?;

    init_subscriber(args.log_path());

    let (term_tx, term_rx) = bounded(1);
    let term_listener = TerminalEventListener::build()?;
    term_listener.run(term_tx);

    let (serv_tx, serv_rx) = bounded(16);

    let mut selections = Select::new();
    let term_index = selections.recv(&term_rx);
    let serv_index = selections.recv(&serv_rx);
    // let mut app = App::default();
    //
    // app.run();

    Ok(())
}

/// Initialize the default global tracing subscriber.
fn init_subscriber(log_path: &str) {
    let appender = rolling::never(".", log_path);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("CAKESTRY_LOG"))
        .with_ansi(false)
        .with_writer(appender)
        .init();
}
