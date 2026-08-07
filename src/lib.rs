use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use ratatui::DefaultTerminal;

mod client;
mod config;
mod render;
mod session;
mod terminal;

use client::Client;
use config::Config;
use render::SessionRenderer;
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
        let mut session_render = SessionRenderer::default();

        loop {
            self.terminal
                .draw(|frame| session_render.render(&self.session, frame))?;

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
