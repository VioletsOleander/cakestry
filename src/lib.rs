use crossterm::event::{Event, KeyCode, KeyModifiers};

mod client;
mod config;
mod session;
mod terminal;

use client::Client;
use config::Config;
use session::Session;
use terminal::Terminal;

pub struct App {
    config: Config,
    client: Client,
    session: Session,
    terminal: Terminal,
}

impl Default for App {
    fn default() -> Self {
        let config = Config::from_file(".cakestry/config.toml");
        let client = Client::new(
            config
                .get_provider(config.default_provider())
                .expect("The default_provider should be a valid provider's name."),
        );

        App {
            config,
            client,
            session: Session::default(),
            terminal: Terminal::default(),
        }
    }
}

impl App {
    pub fn run(&mut self) {
        loop {
            self.terminal.draw(&self.session, &self.client);

            match self.terminal.read_event() {
                Event::Key(key) => match key.code {
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Enter if key.modifiers == KeyModifiers::NONE => {
                        self.session.handle_key(key);
                    }
                    _ => {
                        self.session.handle_key(key);
                    }
                },
                Event::Mouse(mouse) => {
                    self.session.handle_mouse(mouse);
                }
                _ => (),
            };
        }
    }
}
