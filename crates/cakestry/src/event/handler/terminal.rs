use anyhow::Result;
use async_openai::types::responses::{
    EasyInputMessage, InputItem, InputParam, Role, Tool as OpenAITool,
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEventKind};
use crossterm::terminal;

use crate::command::CommandHandler;
use crate::config::Tool;
use crate::event::service::{ResponseEventService, TickEventService};
use crate::state::State;
use crate::state::session::Mode;
use crate::state::session::record::Record;

pub struct TerminalEventHandler<'a> {
    resp_service: &'a ResponseEventService,
    tick_service: &'a TickEventService,
}

impl<'a> TerminalEventHandler<'a> {
    pub fn new(resp_service: &'a ResponseEventService, tick_service: &'a TickEventService) -> Self {
        Self {
            resp_service,
            tick_service,
        }
    }

    pub fn handle(&self, event: Event, state: &mut State) -> Result<()> {
        match event {
            Event::Key(key) => match state.session().mode {
                Mode::Prompt => self.handle_prompt_key(key, state),
                Mode::Command => self.handle_command_key(key, state),
                Mode::Scroll => self.handle_scroll_key(key, state)?,
            },
            Event::Mouse(mouse) => match mouse.kind {
                // Step size 3 is comfortable, step size 1 feels too sluggish.
                MouseEventKind::ScrollUp => scroll_up(state, 3),
                MouseEventKind::ScrollDown => scroll_down(state, 3),
                _ => (),
            },
            _ => (),
        };

        Ok(())
    }

    fn handle_prompt_key(&self, key: KeyEvent, state: &mut State) {
        match key.modifiers {
            KeyModifiers::CONTROL => match key.code {
                // CTRL-c: Exit prompt mode, enter command mode.
                KeyCode::Char('c') => state.session().mode = Mode::Command,
                // CTRL-s: Exit prompt mode, enter scroll mode.
                KeyCode::Char('s') => state.session().mode = Mode::Scroll,
                // CTRL-j: Break current line.
                KeyCode::Char('j') => state.session().prompt_buffer.break_line(),
                // CTRL-w: Delete a word backward.
                KeyCode::Char('w') => state.session().prompt_buffer.delete_word_backward(),
                // CTRL-u: Delete a line backward.
                KeyCode::Char('u') => state.session().prompt_buffer.delete_line_backward(),
                _ => (),
            },
            KeyModifiers::SHIFT => {
                if let KeyCode::Char(ch) = key.code {
                    state.session().prompt_buffer.insert_char(ch)
                }
            }
            KeyModifiers::NONE => match key.code {
                // Enter: Submit prompt
                KeyCode::Enter => self.submit_prompt(state),
                KeyCode::Char(ch) => state.session().prompt_buffer.insert_char(ch),
                KeyCode::Delete => state.session().prompt_buffer.delete_char(),
                KeyCode::Backspace => state.session().prompt_buffer.delete_prev_char(),
                KeyCode::Left => state.session().prompt_buffer.move_cursor_left(),
                KeyCode::Right => state.session().prompt_buffer.move_cursor_right(),
                KeyCode::Up => state.session().prompt_buffer.move_cursor_up(),
                KeyCode::Down => state.session().prompt_buffer.move_cursor_down(),
                _ => (),
            },
            _ => (),
        }
    }

    fn handle_command_key(&self, key: KeyEvent, state: &mut State) {
        match key.modifiers {
            KeyModifiers::CONTROL => match key.code {
                // CTRL-s: Exit command mode, enter scroll mode.
                KeyCode::Char('s') => {
                    state.command_buffer.clear();
                    state.session().mode = Mode::Scroll;
                }
                // CTRL-j: Break current line.
                KeyCode::Char('j') => state.command_buffer.break_line(),
                // CTRL-w: Delete a word backward.
                KeyCode::Char('w') => state.command_buffer.delete_word_backward(),
                // CTRL-u: Delete a line backward.
                KeyCode::Char('u') => state.command_buffer.delete_line_backward(),
                _ => (),
            },
            KeyModifiers::SHIFT if let KeyCode::Char(ch) = key.code => {
                state.command_buffer.insert_char(ch)
            }
            KeyModifiers::SHIFT => {}
            KeyModifiers::NONE => match key.code {
                // Esc: Exit command mode, enter prompt mode.
                KeyCode::Esc => {
                    state.command_buffer.clear();
                    state.session().mode = Mode::Prompt;
                }
                // Enter: Submit command.
                KeyCode::Enter => self.submit_command(state),
                // Backspace: Delete prev char or back to prompt mode.
                KeyCode::Backspace => {
                    if state.command_buffer.is_empty() {
                        state.session().mode = Mode::Prompt;
                    } else {
                        state.command_buffer.delete_prev_char();
                    }
                }
                KeyCode::Char(ch) => state.command_buffer.insert_char(ch),
                KeyCode::Delete => state.command_buffer.delete_char(),
                KeyCode::Left => state.command_buffer.move_cursor_left(),
                KeyCode::Right => state.command_buffer.move_cursor_right(),
                _ => (),
            },
            _ => (),
        }
    }

    fn handle_scroll_key(&self, key: KeyEvent, state: &mut State) -> Result<()> {
        match key.modifiers {
            KeyModifiers::CONTROL => match key.code {
                // CTRL-c: Exit scroll mode, enter command mode.
                KeyCode::Char('c') => state.session().mode = Mode::Command,
                // CTRL-u: Scroll up half screen.
                KeyCode::Char('u') => scroll_up(state, (terminal::size()?.1 / 2) as usize),
                // CTRL-d: Scroll down half screen.
                KeyCode::Char('d') => scroll_down(state, (terminal::size()?.1 / 2) as usize),
                // Ctrl-y: Scroll up 1/4 screen.
                KeyCode::Char('y') => scroll_up(state, (terminal::size()?.1 / 4) as usize),
                // Ctrl-e: Scroll down 1/4 screen.
                KeyCode::Char('e') => scroll_down(state, (terminal::size()?.1 / 4) as usize),
                // Ctrl-k: Scroll up 1 line.
                KeyCode::Char('k') => scroll_up(state, 1),
                // Ctrl-j: Scroll down 1 line.
                KeyCode::Char('j') => scroll_down(state, 1),
                _ => (),
            },
            KeyModifiers::NONE => match key.code {
                // Esc: Exit scroll mode, back to prompt mode.
                KeyCode::Esc => state.session().mode = Mode::Prompt,
                // i: Exit scroll mode, back to prompt mode.
                KeyCode::Char('i') => state.session().mode = Mode::Prompt,
                // Up: Scroll up one line.
                KeyCode::Up => scroll_up(state, 1),
                // Down: Scroll down one line.
                KeyCode::Down => scroll_down(state, 1),
                _ => (),
            },
            _ => (),
        };

        Ok(())
    }

    fn submit_prompt(&self, state: &mut State) {
        if let Some(prompt) = state.session().prompt_buffer.take_content() {
            state.session().records.push(Record::Prompt(prompt));

            self.resp_service.create_responses(
                make_input_param(state),
                make_tools(state),
                make_model(state),
            );
            self.tick_service.run();
        }
    }

    fn submit_command(&self, state: &mut State) {
        if let Some(command) = state.command_buffer.take_content() {
            CommandHandler::handle(command, state);
        }
    }
}

fn scroll_up(state: &mut State, delta: usize) {
    state.session().scroll.up(delta);
}

fn scroll_down(state: &mut State, delta: usize) {
    state.session().scroll.down(delta);
}

fn make_input_param(state: &mut State) -> InputParam {
    let mut input_items: Vec<InputItem> = Vec::with_capacity(1 + state.session().records.len());

    input_items.push(
        EasyInputMessage {
            role: Role::System,
            content: state.session().system_prompt.clone().into(),
            ..Default::default()
        }
        .into(),
    );

    for record in state.session().records.iter() {
        input_items.push(record.into());
    }

    input_items.into()
}

fn make_tools(state: &mut State) -> Vec<OpenAITool> {
    let mut tools: Vec<OpenAITool> = Vec::new();

    for tool in state.tools.iter() {
        match tool {
            Tool::Function { enabled, .. } => {
                if *enabled {
                    tools.push(tool.into())
                }
            }
            Tool::WebSearch { enabled, .. } => {
                if *enabled {
                    tools.push(tool.into());
                }
            }
        }
    }

    tools
}

fn make_model(state: &mut State) -> String {
    let session = &state.sessions[state.session_index];
    let provider = &state.providers[session.provider_index];

    provider.models[session.model_index].clone()
}
