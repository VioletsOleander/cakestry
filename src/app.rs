use crossbeam_channel::{Select, bounded};

use super::config::Config;
use super::service::Service;
use super::session::{Exchange, Session};
use super::terminal::{KeyEvent, MouseEvent, Terminal};

mod command;
mod event;
mod state;

// use command::CommandLine;
use state::{Mode, State};

pub struct App {
    state: State,
    /// Terminal, for user interaction (event reading and tui rendering).
    terminal: Terminal,
    /// LLM completion service over HTTP.
    service: Service,
    session: Session,
    // command_line: CommandLine,
}

enum Action {
    LaunchRequest,
    // LaunchCommand,
    SwitchMode(Mode),
    HandleKey(KeyEvent),
    HandleMouse(MouseEvent),
    None,
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
                        // Action::LaunchCommand(command) => {
                        //     let command = CommandParser::parse(command);
                        // }
                        Action::SwitchMode(mode) => {
                            self.state.set_mode(mode);
                        }
                        Action::HandleKey(key) => {
                            self.session.handle_key(key);
                        }
                        Action::HandleMouse(mouse) => {
                            self.session.handle_mouse(mouse);
                        }
                        Action::None => (),
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
}

impl Default for App {
    fn default(config: Config) -> Self {
        let service = Service::new(
            config
                .get_provider(config.default_provider())
                .expect("The default_provider should be a valid provider's name."),
        );

        App {
            service,
            session: Session::default(),
            terminal: Terminal::default(),
            state: State::default(),
        }
    }
}
