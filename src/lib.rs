use crossterm::event::{Event, KeyCode, KeyModifiers};
use tokio::runtime::{Builder, Runtime};

mod config;
mod service;
mod session;
mod terminal;

use config::Config;
use service::Service;
use session::Session;
use terminal::Terminal;

pub struct App {
    config: Config,
    service: Service,
    session: Session,
    terminal: Terminal,
    runtime: Runtime,
}

impl App {
    pub fn run(&mut self) {
        loop {
            self.terminal.draw(&self.session, &self.service);

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
                            .service
                            .make_request(self.session.exchanges(), &user_input);

                        let handle = self.service.send_request(request, &self.runtime);

                        // self.session
                        //     .last_exchange_mut()
                        //     .set_reply(response.output_text().unwrap_or_default());
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
        let service = Service::new(
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
            service,
            session: Session::default(),
            terminal: Terminal::default(),
            runtime,
        }
    }
}
