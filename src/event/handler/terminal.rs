use anyhow::Result;
use async_openai::types::responses::{
    EasyInputContent, EasyInputMessage, EasyInputMessageArgs, Role,
};
use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEventKind};

use crate::event::service::ResponseEventService;
use crate::state::{Mode, State};

pub struct TerminalEventHandler {
    service: ResponseEventService,
}

impl TerminalEventHandler {
    pub fn new(service: ResponseEventService) -> Self {
        TerminalEventHandler { service }
    }

    pub fn handle(&mut self, event: Event, state: &mut State) {
        match event {
            Event::Key(key) => match key.modifiers {
                KeyModifiers::CONTROL => match key.code {
                    // CTRL-c: Enter command mode.
                    KeyCode::Char('c') => state.set_mode(Mode::Command),
                    // CTRL-j: Break current line.
                    KeyCode::Char('j') => state.active_buffer().break_line(),
                    // CTRL-w: Delete a word backward.
                    KeyCode::Char('w') => state.active_buffer().delete_word_backward(),
                    // CTRL-u: Delete a line backward.
                    KeyCode::Char('u') => state.active_buffer().delete_line_backward(),
                    _ => (),
                },
                KeyModifiers::SHIFT => {
                    if let KeyCode::Char(ch) = key.code {
                        state.active_buffer().insert_char(ch)
                    }
                }
                KeyModifiers::NONE => match key.code {
                    // Esc: Exit command mode, back to prompt mode.
                    KeyCode::Esc => {
                        if let Mode::Command = state.mode() {
                            state.set_mode(Mode::Prompt);
                        }
                    }
                    // Enter: Submit prompt or command.
                    KeyCode::Enter => match state.mode() {
                        Mode::Prompt => self.try_submit_prompt(state),
                        Mode::Command => {
                            let buffer = state.active_buffer();
                            if buffer.is_empty() {
                                return;
                            }
                        }
                    },
                    KeyCode::Char(ch) => state.active_buffer().insert_char(ch),
                    KeyCode::Delete => state.active_buffer().delete_char(),
                    KeyCode::Backspace => state.active_buffer().delete_prev_char(),
                    KeyCode::Left => state.active_buffer().move_cursor_left(),
                    KeyCode::Right => state.active_buffer().move_cursor_right(),
                    _ => (),
                },
                _ => (),
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => state.increment_scroll_offset(),
                MouseEventKind::ScrollDown => state.decrement_scroll_offset(),
                _ => (),
            },
            _ => (),
        }
    }

    fn try_submit_prompt(&self, state: &mut State) {
        if state.awaiting_response() {
            return;
        }

        let buffer = state.prompt_buffer_mut();
        if let Some(content) = buffer.take_content() {
            let exchanges = state.exchanges_mut();
            // Each exchange 2 message + 1 system prompt + 1 user input.
            let mut messages = Vec::with_capacity(2 * exchanges.len() + 2);

            messages.push(make_message(
                Role::System,
                "You are a helpful assistant".to_string(),
            ));
            for exchange in exchanges {
                messages.push(make_message(Role::User, exchange.query().to_string()));
                messages.push(make_message(Role::Assistant, exchange.reply().to_string()));
            }
            messages.push(make_message(Role::User, content));
        }
    }
}

fn make_message(role: Role, content: String) -> EasyInputMessage {
    // Here, `expect` is used because message build error implies code logic error
    // and therefore is unrecoverable.
    EasyInputMessageArgs::default()
        .role(role)
        .content(EasyInputContent::Text(content))
        .build()
        .expect("role and content should be enough to make EasyInputMessage.")
}
