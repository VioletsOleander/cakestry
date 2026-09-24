use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::event::service::ResponseEventService;
use crate::state::State;

enum BufferType {
    Prompt,
    Command,
}

pub struct TerminalEventHandler {
    resp_service: ResponseEventService,
    wait_resp: bool,
    active_buffer: BufferType,
}

impl TerminalEventHandler {
    pub fn new(resp_service: ResponseEventService) -> Self {
        TerminalEventHandler {
            resp_service,
            wait_resp: false,
            active_buffer: BufferType::Prompt,
        }
    }

    pub fn handle(&mut self, event: Event, state: &mut State) {
        match event {
            Event::Key(key) => match key.modifiers {
                KeyModifiers::CONTROL => match key.code {
                    // CTRL-Esc: Exit command mode, back to prompt mode.
                    KeyCode::Esc => match self.active_buffer {
                        BufferType::Command => self.active_buffer = BufferType::Prompt,
                        _ => (),
                    },
                    // CTRL-c: Enter command mode.
                    KeyCode::Char('c') => self.active_buffer = BufferType::Command,
                    _ => (),
                },
                KeyModifiers::SHIFT => match key.code {
                    // SHIFT-Esc: Exit command mode, back to prompt mode.
                    KeyCode::Esc => match self.active_buffer {
                        BufferType::Command => self.active_buffer = BufferType::Prompt,
                        _ => (),
                    },
                    _ => (),
                },
                KeyModifiers::NONE => match key.code {
                    // Esc: Exit command mode, back to prompt mode.
                    KeyCode::Esc => match self.active_buffer {
                        BufferType::Command => self.active_buffer = BufferType::Prompt,
                        _ => (),
                    },
                    // Enter: Submit prompt or command.
                    KeyCode::Enter => match self.active_buffer {
                        BufferType::Prompt => {
                            if self.wait_resp || state.prompt_buffer_mut().is_empty() {
                                return;
                            }

                            println!("placeholder");
                        }
                        BufferType::Command => (),
                    },
                    _ => (),
                },
                _ => (),
            },
            Event::Mouse(_) => {
                println!("placeholder");
            }
            _ => (),
        }
    }
}
