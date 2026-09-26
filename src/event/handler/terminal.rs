use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEventKind};

use crate::event::service::ResponseEventService;
use crate::state::State;
use crate::state::buffer::{TextBuffer, TextBufferKind};

pub struct TerminalEventHandler {
    service: ResponseEventService,
    wait_response: bool,
    active_buffer_kind: TextBufferKind,
}

impl TerminalEventHandler {
    pub fn new(service: ResponseEventService) -> Self {
        TerminalEventHandler {
            service,
            wait_response: false,
            active_buffer_kind: TextBufferKind::Prompt,
        }
    }

    pub fn handle(&mut self, event: Event, state: &mut State) {
        match event {
            Event::Key(key) => match key.modifiers {
                KeyModifiers::CONTROL => match key.code {
                    // CTRL-Esc: Exit command mode, back to prompt mode.
                    KeyCode::Esc => {
                        if let TextBufferKind::Command = self.active_buffer_kind {
                            self.active_buffer_kind = TextBufferKind::Prompt
                        }
                    }
                    // CTRL-c: Enter command mode.
                    KeyCode::Char('c') => self.active_buffer_kind = TextBufferKind::Command,
                    // CTRL-j: Break current line.
                    KeyCode::Char('j') => self.get_active_buffer(state).break_line(),
                    // CTRL-w: Delete a word backward.
                    KeyCode::Char('w') => self.get_active_buffer(state).delete_word_backward(),
                    // CTRL-u: Delete a line backward.
                    KeyCode::Char('u') => self.get_active_buffer(state).delete_line_backward(),
                    _ => (),
                },
                KeyModifiers::SHIFT => match key.code {
                    // SHIFT-Esc: Exit command mode, back to prompt mode.
                    KeyCode::Esc => {
                        if let TextBufferKind::Command = self.active_buffer_kind {
                            self.active_buffer_kind = TextBufferKind::Prompt
                        }
                    }
                    KeyCode::Char(ch) => self.get_active_buffer(state).insert_char(ch),
                    _ => (),
                },
                KeyModifiers::NONE => match key.code {
                    // Esc: Exit command mode, back to prompt mode.
                    KeyCode::Esc => {
                        if let TextBufferKind::Command = self.active_buffer_kind {
                            self.active_buffer_kind = TextBufferKind::Prompt
                        }
                    }
                    // Enter: Submit prompt or command.
                    KeyCode::Enter => match self.active_buffer_kind {
                        TextBufferKind::Prompt => {
                            if self.wait_response || state.prompt_buffer_mut().is_empty() {
                                return;
                            }

                            todo!();
                        }
                        TextBufferKind::Command => (),
                    },
                    KeyCode::Char(ch) => self.get_active_buffer(state).insert_char(ch),
                    KeyCode::Delete => self.get_active_buffer(state).delete_char(),
                    KeyCode::Backspace => self.get_active_buffer(state).delete_prev_char(),
                    KeyCode::Left => self.get_active_buffer(state).move_cursor_left(),
                    KeyCode::Right => self.get_active_buffer(state).move_cursor_right(),
                    _ => (),
                },
                _ => (),
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => {
                    state.set_scroll_offset(state.scroll_offset().saturating_add(1));
                }
                MouseEventKind::ScrollDown => {
                    state.set_scroll_offset(state.scroll_offset().saturating_sub(1));
                }
                _ => (),
            },
            _ => (),
        }
    }

    fn get_active_buffer<'a>(&self, state: &'a mut State) -> &'a mut TextBuffer {
        match self.active_buffer_kind {
            TextBufferKind::Prompt => state.prompt_buffer_mut(),
            TextBufferKind::Command => state.command_buffer_mut(),
        }
    }
}
