use crossbeam_channel::{Select, bounded};

use super::config::Config;
use super::service::{Service, ServiceEvent};
use super::session::{Exchange, Session};
use super::terminal::{KeyCode, KeyModifiers, Terminal, TerminalEvent};

mod action;
mod state;

use action::Action;
use state::{Mode, State};

pub struct App {
    /// Config for the whole app.
    config: Config,
    /// LLM completion service over HTTP.
    service: Service,
    /// Session data and state storage.
    session: Session,
    /// Terminal, for user interaction (event reading and tui rendering).
    terminal: Terminal,
    state: State,
}

impl App {
    pub fn run(&mut self) {
        let (term_tx, term_rx) = bounded(1);
        let (serv_tx, serv_rx) = bounded(16);

        let mut selections = Select::new();
        let term_index = selections.recv(&term_rx);
        let serv_index = selections.recv(&serv_rx);

        self.terminal.spawn_listener(term_tx);

        loop {
            self.terminal.draw(&self.session, &self.service);

            let operation = selections.select();

            match operation.index() {
                i if i == term_index => {
                    let event = operation.recv(&term_rx).expect(
                        "The terminal event channel should keep alive before the receiver's drop.",
                    );

                    match self.handle_term_event(event) {
                        Action::LaunchRequest => {
                            let query = self.session.take_user_input();
                            let request =
                                self.service.make_request(self.session.exchanges(), &query);

                            self.session
                                .add_exchange(Exchange::new(query, String::from("Waiting...")));
                            self.service.make_responses(request, serv_tx.clone());
                        }
                        Action::LaunchCommand => (),
                        Action::SwitchMode(mode) => {
                            self.state.set_mode(mode);
                        }
                        Action => (),
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

    fn handle_term_event(&mut self, event: TerminalEvent) -> Action {
        match event {
            TerminalEvent::Key(key) => match key.modifiers {
                KeyModifiers::CONTROL => match key.code {
                    KeyCode::Char('c') => Action::SwitchMode(Mode::Command),
                    KeyCode::Esc => match self.state.mode() {
                        Mode::Normal => Action::None,
                        Mode::Command => Action::SwitchMode(Mode::Normal),
                    },
                    _ => {
                        self.session.handle_key(key);
                        Action::None
                    }
                },
                KeyModifiers::SHIFT => match key.code {
                    KeyCode::Esc => match self.state.mode() {
                        Mode::Normal => Action::None,
                        Mode::Command => Action::SwitchMode(Mode::Normal),
                    },
                    _ => {
                        self.session.handle_key(key);
                        Action::None
                    }
                },
                KeyModifiers::NONE => match key.code {
                    KeyCode::Esc => match self.state.mode() {
                        Mode::Normal => Action::None,
                        Mode::Command => Action::SwitchMode(Mode::Normal),
                    },
                    KeyCode::Enter => match self.state.mode() {
                        Mode::Normal => {
                            if self.state.wait() {
                                return Action::None;
                            }

                            if self.session.user_input().is_empty() {
                                return Action::None;
                            }

                            Action::LaunchRequest
                        }
                        Mode::Command => Action::LaunchCommand,
                    },
                    _ => {
                        self.session.handle_key(key);
                        Action::None
                    }
                },
                _ => Action::None,
            },
            TerminalEvent::Mouse(mouse) => {
                self.session.handle_mouse(mouse);
                Action::None
            }
        }
    }

    fn handle_serv_event(&mut self, event: ServiceEvent) {
        match event {
            ServiceEvent::ResponseStart => {
                self.state.set_wait(true);
            }
            ServiceEvent::ResponseComplete
            | ServiceEvent::ResponseFail
            | ServiceEvent::ResponseInComplete => {
                self.state.set_wait(false);
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
            state: State::default(),
        }
    }
}
