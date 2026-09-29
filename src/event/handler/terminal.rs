use async_openai::types::responses::{
    EasyInputContent, EasyInputMessage, EasyInputMessageArgs, Role,
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEventKind};

use crate::command::CommandHandler;
use crate::event::service::ResponseEventService;
use crate::state::{Mode, State};

pub struct TerminalEventHandler {
    service: ResponseEventService,
}

impl TerminalEventHandler {
    pub fn new(service: ResponseEventService) -> Self {
        TerminalEventHandler { service }
    }

    pub fn handle(&self, event: Event, state: &mut State) {
        match event {
            Event::Key(key) => match state.mode {
                Mode::Prompt => self.handle_prompt_key(key, state),
                Mode::Command => self.handle_command_key(key, state),
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => {
                    state.scroll_offset = state.scroll_offset.saturating_sub(1);
                }

                MouseEventKind::ScrollDown => {
                    state.scroll_offset = state.scroll_offset.saturating_add(1);
                }
                _ => (),
            },
            _ => (),
        };
    }

    fn handle_prompt_key(&self, key: KeyEvent, state: &mut State) {
        match key.modifiers {
            KeyModifiers::CONTROL => match key.code {
                // CTRL-c: Enter command mode.
                KeyCode::Char('c') => state.mode = Mode::Command,
                // CTRL-j: Break current line.
                KeyCode::Char('j') => state.prompt_buffer.break_line(),
                // CTRL-w: Delete a word backward.
                KeyCode::Char('w') => state.prompt_buffer.delete_word_backward(),
                // CTRL-u: Delete a line backward.
                KeyCode::Char('u') => state.prompt_buffer.delete_line_backward(),
                _ => (),
            },
            KeyModifiers::SHIFT => {
                if let KeyCode::Char(ch) = key.code {
                    state.prompt_buffer.insert_char(ch)
                }
            }
            KeyModifiers::NONE => match key.code {
                // Enter: Submit prompt
                KeyCode::Enter => self.try_submit_prompt(state),
                KeyCode::Char(ch) => state.prompt_buffer.insert_char(ch),
                KeyCode::Delete => state.prompt_buffer.delete_char(),
                KeyCode::Backspace => state.prompt_buffer.delete_prev_char(),
                KeyCode::Left => state.prompt_buffer.move_cursor_left(),
                KeyCode::Right => state.prompt_buffer.move_cursor_right(),
                _ => (),
            },
            _ => (),
        }
    }

    fn handle_command_key(&self, key: KeyEvent, state: &mut State) {
        match key.modifiers {
            KeyModifiers::CONTROL => match key.code {
                // CTRL-j: Break current line.
                KeyCode::Char('j') => state.command_buffer.break_line(),
                // CTRL-w: Delete a word backward.
                KeyCode::Char('w') => state.command_buffer.delete_word_backward(),
                // CTRL-u: Delete a line backward.
                KeyCode::Char('u') => state.command_buffer.delete_line_backward(),
                _ => (),
            },
            KeyModifiers::SHIFT => {
                if let KeyCode::Char(ch) = key.code {
                    state.command_buffer.insert_char(ch)
                }
            }
            KeyModifiers::NONE => match key.code {
                // Esc: Exit command mode, back to prompt mode.
                KeyCode::Esc => state.mode = Mode::Prompt,
                // Enter: Submit command.
                KeyCode::Enter => self.try_submit_command(state),
                KeyCode::Char(ch) => state.command_buffer.insert_char(ch),
                KeyCode::Delete => state.command_buffer.delete_char(),
                KeyCode::Backspace => state.command_buffer.delete_prev_char(),
                KeyCode::Left => state.command_buffer.move_cursor_left(),
                KeyCode::Right => state.command_buffer.move_cursor_right(),
                _ => (),
            },
            _ => (),
        }
    }

    fn try_submit_prompt(&self, state: &mut State) {
        if !state.awaiting_response
            && let Some(prompt) = state.prompt_buffer.take_content()
        {
            let messages = make_messages(prompt, state);
            let model = state.providers[state.provider_index].model.clone();

            self.service.create_responses(messages, model);
            state.awaiting_response = true
        }
    }

    fn try_submit_command(&self, state: &mut State) {
        if let Some(command) = state.command_buffer.take_content() {
            CommandHandler::handle(command, state);
        }
    }
}

fn make_messages(prompt: String, state: &State) -> Vec<EasyInputMessage> {
    let mut messages = Vec::with_capacity(2 * state.exchanges.len() + 2);

    messages.push(make_message(Role::System, state.system_prompt.to_string()));
    for exchange in &state.exchanges {
        messages.push(make_message(Role::User, exchange.prompt.clone()));
        messages.push(make_message(Role::Assistant, exchange.response.clone()));
    }
    messages.push(make_message(Role::User, prompt));

    messages
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
