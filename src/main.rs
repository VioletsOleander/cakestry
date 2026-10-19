use anyhow::Result;
use clap::Parser;
use clap::builder::Styles;
use crossbeam_channel::{bounded, select};

mod command;
mod config;
mod event;
mod state;
mod tui;

use config::Config;
use event::handler::{
    ResponseEventHandler, TerminalEventHandler, TickEventHandler, ToolEventHandler,
};
use event::service::{ResponseEventService, TerminalEventService, TickEventService, ToolService};
use state::State;
use tui::Tui;

#[derive(Parser)]
#[command(version, about)]
#[command(styles = Styles::default())]
struct Args {
    /// Relative path to the config file.
    #[arg(long, default_value = ".cakestry/config.toml")]
    config_path: String,
    /// Relative path to the log file.
    #[arg(long, default_value = "cakestry.log")]
    log_path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::from_file(&args.config_path)?;

    init_subscriber(&args.log_path);

    let mut state = State::build(config)?;
    let mut tui = Tui::build()?;

    let (term_tx, term_rx) = bounded(1);
    let (resp_tx, resp_rx) = bounded(16);
    let (tool_tx, tool_rx) = bounded(1);
    let (tick_tx, tick_rx) = bounded(1);
    let (tickstop_tx, tickstop_rx) = bounded(1);

    let term_service = TerminalEventService::build()?;
    let resp_service = ResponseEventService::build(&state, resp_tx)?;
    let tool_service = ToolService::new();
    let tick_service = TickEventService::new(tick_tx, tickstop_rx);

    let term_handler = TerminalEventHandler::new(resp_service, tick_service);
    let resp_handler = ResponseEventHandler::new(tool_service, tickstop_tx);
    let tool_handler = ToolEventHandler::new();
    let tick_handler = TickEventHandler::new();

    term_service.run(term_tx);

    while !state.should_exit {
        tui.render(&state)?;

        select! {
            recv(term_rx) -> result => {
                // The second error is io::Error, which is unrecoverable.
                let event = result??;
                term_handler.handle(event, &mut state)?;
            },
            recv(resp_rx) -> result => {
                let event = result?;
                resp_handler.handle(event, &mut state);
            },
            recv(tick_rx) -> result => {
                let event = result?;
                tick_handler.handle(event, &mut state);
            },
            recv(tool_rx) -> result => {
                let event = result?;
                tool_handler.handle(event, &mut state);
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
