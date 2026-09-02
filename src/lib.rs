use crossbeam_channel::{Select, bounded};

mod config;
mod service;
mod session;
mod terminal;

use config::Config;
use service::{Service, ServiceEvent};
use session::{Exchange, Session};
use terminal::{Terminal, TerminalEvent};

pub struct App {
    /// Config for the whole app.
    config: Config,
    /// LLM completion service over HTTP.
    service: Service,
    /// Session data and state storage.
    session: Session,
    /// Terminal, for user interaction (event reading and tui rendering).
    terminal: Terminal,
    /// Whether 'TerminalEvent::Confirm' should work.
    confirm_locked: bool,
}

impl App {
    pub fn run(&mut self) {
        let (term_tx, term_rx) = bounded(1);
        let (serv_tx, serv_rx) = bounded(16);

        let mut selections = Select::new();
        let term_index = selections.recv(&term_rx);
        let serv_index = selections.recv(&serv_rx);

        self.terminal.spawn_event_listener(term_tx);

        loop {
            self.terminal.draw(&self.session, &self.service);

            let operation = selections.select();
            match operation.index() {
                i if i == term_index => {
                    let event = operation.recv(&term_rx).expect(
                        "The terminal event channel should keep alive before the receiver's drop.",
                    );

                    match event {
                        // Special cases.
                        TerminalEvent::Exit => {
                            break;
                        }
                        TerminalEvent::Confirm => {
                            if self.confirm_locked {
                                continue;
                            }

                            if self.session.user_input().is_empty() {
                                continue;
                            }

                            let query = self.session.take_user_input();
                            let request =
                                self.service.make_request(self.session.exchanges(), &query);

                            self.session
                                .add_exchange(Exchange::new(query, String::from("Waiting...")));
                            self.service.make_responses(request, serv_tx.clone());
                        }
                        // Other cases.
                        _ => {
                            self.handle_term_event(event);
                        }
                    }
                }
                i if i == serv_index => {
                    let event = operation.recv(&serv_rx).expect(
                        "The service event channel should keep alive before the receiver's drop.",
                    );

                    self.handle_serv_event(event);
                }
                _ => unreachable!(),
            }
        }
    }

    fn handle_term_event(&mut self, event: TerminalEvent) {
        match event {
            TerminalEvent::Key(key) => {
                self.session.handle_key(key);
            }
            TerminalEvent::Mouse(mouse) => {
                self.session.handle_mouse(mouse);
            }
            _ => (),
        }
    }

    fn handle_serv_event(&mut self, event: ServiceEvent) {
        match event {
            ServiceEvent::ResponseStart => {
                self.confirm_locked = true;
            }
            ServiceEvent::ResponseComplete
            | ServiceEvent::ResponseFail
            | ServiceEvent::ResponseInComplete => {
                self.confirm_locked = false;
            }
            ServiceEvent::ReasoningStart => {
                self.session
                    .last_exchange()
                    .set_reply(String::from("Thinking..."));
            }
            ServiceEvent::ReasoningComplete(_) => (), // No sure where to store reasoning content now.
            ServiceEvent::MessageStart => {
                self.session.last_exchange().set_reply(String::new());
            }
            ServiceEvent::MessageDeltaText(delta) => {
                self.session.last_exchange().push_to_reply(&delta);
            }
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

        App {
            config,
            service,
            session: Session::default(),
            terminal: Terminal::default(),
            confirm_locked: false,
        }
    }
}
