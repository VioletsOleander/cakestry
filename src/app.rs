use crossbeam_channel::{Select, bounded};

use super::config::Config;
use super::service::{Service, ServiceEvent};
use super::session::{Exchange, Session};
use super::terminal::{KeyCode, KeyModifiers, Terminal, TerminalEvent};

pub struct App {
    /// Config for the whole app.
    config: Config,
    /// LLM completion service over HTTP.
    service: Service,
    /// Session data and state storage.
    session: Session,
    /// Terminal, for user interaction (event reading and tui rendering).
    terminal: Terminal,
    mode: Mode,
}

enum Action {
    MakeRequest,
    AbortResponse,
    ExecuteCommand(Command),
    SwitchMode(Mode),
    None,
}

enum Mode {
    Request,
    Response,
    Command,
}

enum Command {
    ExitApp,
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

    // fn handle_term_event(&mut self, event: TerminalEvent) -> Action {
    //     match event {
    //         TerminalEvent::Key(key) => match key.modifiers {
    //             KeyModifiers::CONTROL => match key.code {
    //                 KeyCode::Char('c') => {
    //                     self.prev_mode = self.mode;
    //                     Action::SwitchMode(Mode::Command)
    //                 }
    //                 KeyCode::Esc => match self.mode {
    //                     Mode::Request => Action::None,
    //                     Mode::Response => Action::AbortResponse,
    //                     Mode::Command => {
    //                         let mode = self.prev_mode;
    //                         self.prev_mode = self.mode;
    //
    //                         Action::SwitchMode(mode)
    //                     }
    //                 },
    //                 _ => {
    //                     self.session.handle_key(key);
    //                     Action::None
    //                 }
    //             },
    //             KeyModifiers::SHIFT => match key.code {
    //                 KeyCode::Esc => {
    //                     self.prev_mode = self.mode;
    //
    //                     Action::SwitchMode(Mode::Normal)
    //                 }
    //                 _ => {
    //                     self.session.handle_key(key);
    //                     Action::None
    //                 }
    //             },
    //             KeyModifiers::NONE => match key.code {
    //                 KeyCode::Esc => Action::SwitchMode(Mode::Normal),
    //                 // KeyCode::Enter => {
    //                 //     match self.
    //                 //     if self.confirm_locked {
    //                 //         continue;
    //                 //     }
    //                 //
    //                 //     if self.session.user_input().is_empty() {
    //                 //         continue;
    //                 //     }
    //                 //
    //                 //     let query = self.session.take_user_input();
    //                 //     let request = self.service.make_request(self.session.exchanges(), &query);
    //                 //
    //                 //     self.session
    //                 //         .add_exchange(Exchange::new(query, String::from("Waiting...")));
    //                 //     self.service.make_responses(request, serv_tx.clone());
    //                 // }
    //                 _ => {
    //                     self.session.handle_key(key);
    //                     Action::None
    //                 }
    //             },
    //         },
    //         TerminalEvent::Mouse(mouse) => {
    //             self.session.handle_mouse(mouse);
    //             Action::None
    //         }
    //     }
    // }

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
            mode: Mode::Normal,
        }
    }
}
