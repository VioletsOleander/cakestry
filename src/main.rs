use anyhow::Result;
use clap::Parser;
use crossbeam_channel::{bounded, select};

mod arg;
mod command;
mod config;
mod event;
mod state;

use arg::Args;
use config::Config;
use event::handler::{ResponseEventHandler, TerminalEventHandler};
use event::service::{ResponseEventService, TerminalEventService};
use state::State;

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::from_file(args.config_path())?;

    init_subscriber(args.log_path());

    let mut state = State::build(config)?;

    let (term_tx, term_rx) = bounded(1);
    let term_service = TerminalEventService::build()?;

    let (resp_tx, resp_rx) = bounded(16);
    let resp_service = ResponseEventService::build(&state, resp_tx)?;

    let term_handler = TerminalEventHandler::new(resp_service);
    let resp_handler = ResponseEventHandler::new();

    term_service.run(term_tx);

    while !state.should_exit {
        select! {
            recv(term_rx) -> result => {
                // The second error is io::Error, which is unrecoverable.
                let event = result??;
                term_handler.handle(event, &mut state);
            },
            recv(resp_rx) -> result => {
                let event = result?;
                resp_handler.handle(event, &mut state);
            }
        };
    }

    Ok(())
}

/// Initialize the default global tracing subscriber.
fn init_subscriber(log_path: &str) {
    let appender = tracing_appender::rolling::never(".", log_path);

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_env("CAKESTRY_LOG"))
        .with_ansi(false)
        .with_writer(appender)
        .init();
}
