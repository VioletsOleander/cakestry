use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use ratatui::DefaultTerminal;

mod client;
mod config;
mod session;
mod terminal;

use client::Client;
use config::Config;
use session::Session;

pub struct App {
    config: Config,
    terminal: DefaultTerminal,
    session: Box<Session>,
}

impl Default for App {
    fn default() -> Self {
        let config = Config::from_file(".cakestry/config.toml");
        let terminal = terminal::init();

        let client = Client::new(
            config
                .get_provider(config.default_provider())
                .expect("The default provider should be a valid name."),
        );
        let session = Session::new(client);

        App {
            config,
            terminal,
            session: Box::new(session),
        }
    }
}

impl App {
    pub fn run(&mut self) -> Result<()> {
        loop {
            self.terminal.draw(|frame| self.session.render(frame))?;

            match crossterm::event::read()? {
                Event::Key(key) if key.code == KeyCode::Esc => break,
                event => self.session.handle_event(event),
            };
        }

        self.cleanup();

        Ok(())
    }

    fn cleanup(&mut self) {
        terminal::restore();
    }
}
