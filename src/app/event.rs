use super::state::Mode;
use super::{Action, App};
use crate::service::ServiceEvent;
use crate::terminal::{KeyCode, KeyModifiers, TerminalEvent};

impl App {
    pub(super) fn handle_term_event(&self, event: TerminalEvent) -> Action {
        match event {
            TerminalEvent::Key(key) => match key.modifiers {
                KeyModifiers::CONTROL => match key.code {
                    KeyCode::Char('c') => Action::SwitchMode(Mode::Command),
                    KeyCode::Esc => match self.state.mode() {
                        Mode::Normal => Action::None,
                        Mode::Command => Action::SwitchMode(Mode::Normal),
                    },
                    _ => Action::HandleKey(key),
                },
                KeyModifiers::SHIFT => match key.code {
                    KeyCode::Esc => match self.state.mode() {
                        Mode::Normal => Action::None,
                        Mode::Command => Action::SwitchMode(Mode::Normal),
                    },
                    _ => Action::HandleKey(key),
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
                        // Mode::Command => Action::LaunchCommand,
                        _ => Action::None,
                    },
                    _ => Action::HandleKey(key),
                },
                _ => Action::None,
            },
            TerminalEvent::Mouse(mouse) => Action::HandleMouse(mouse),
        }
    }

    pub(super) fn handle_serv_event(&mut self, event: ServiceEvent) {
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
