use anyhow::Result;
use crossterm::event::{Event, KeyCode};

mod client;
mod config;
mod session;
mod terminal;

/// Session manager, dispatch and delegate work to current session.
#[derive(Default)]
pub struct App {
    session: Box<session::Session>,
}

impl App {
    pub fn run(&mut self) -> Result<()> {
        init_subscriber();

        let config = config::Config::from_file(".cakestry/config.toml");
        let mut terminal = terminal::init();

        loop {
            terminal.draw(|frame| self.session.render(frame))?;

            match crossterm::event::read()? {
                Event::Key(key) if key.code == KeyCode::Esc => break,
                event => self.session.handle_event(event),
            };
        }

        terminal::restore();

        Ok(())
    }
}

/// Initialize the default global tracing subscriber.
fn init_subscriber() {
    let appender = tracing_appender::rolling::never(".", "cakestry.log");
    let subscriber_builder = tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(appender);

    subscriber_builder.init();
}
