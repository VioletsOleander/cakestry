use anyhow::Result;
use clap::Parser;
use crossbeam_channel::{bounded, select};
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod arg;
mod config;
mod event;
mod state;

use arg::Args;
use config::Config;
use event::handler::TerminalEventHandler;
use event::service::{ResponseEventService, TerminalEventService};
use state::State;

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::from_file(args.config_path())?;

    init_subscriber(args.log_path());

    let (term_tx, term_rx) = bounded(1);
    let term_service = TerminalEventService::build()?;

    let (resp_tx, resp_rx) = bounded(16);
    let resp_service = ResponseEventService::build(&config, resp_tx)?;

    let mut state = State::default();
    let mut term_handler = TerminalEventHandler::new(resp_service);
    term_service.run(term_tx);

    loop {
        select! {
            recv(term_rx) -> result => {
                // IO error is unrecoverable, therefore just propagate it.
                let event = result??;
                term_handler.handle(event, &mut state);
            },
            recv(resp_rx) -> result => {
                // let event = result?;


            }
        };

        break;
    }

    Ok(())
}

/// Initialize the default global tracing subscriber.
fn init_subscriber(log_path: &str) {
    let appender = rolling::never(".", log_path);
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("CAKESTRY_LOG"))
        .with_ansi(false)
        .with_writer(appender)
        .init();
}
