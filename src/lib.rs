use crossterm::event::{Event, KeyCode, KeyModifiers};
use tokio::runtime::{Builder, Runtime};

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
    runtime: Runtime,
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
                        let Some(user_input) = self.session.take_user_input() else {
                            continue;
                        };

                        let request = self
                            .client
                            .make_request(self.session.exchanges(), user_input)
                            .expect("Session state should be valid to make a request.");

                        let response = self
                            .runtime
                            .block_on(self.client.send_request(request))
                            .expect("Client should be able to send a request.");

                        self.session
                            .last_exchange_mut()
                            .set_reply(response.output_text().unwrap_or_default());
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

impl Default for App {
    fn default() -> Self {
        let config = Config::from_file(".cakestry/config.toml");
        let client = Client::new(
            config
                .get_provider(config.default_provider())
                .expect("The default_provider should be a valid provider's name."),
        );

        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Current thread should be able to build a runtime on it.");

        App {
            config,
            client,
            session: Session::default(),
            terminal: Terminal::default(),
            runtime,
        }
    }
}
