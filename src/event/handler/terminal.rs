use anyhow::Result;
use async_openai::types::responses::{
    EasyInputContent, EasyInputMessageArgs, FunctionCallOutput, FunctionCallOutputItemParam,
    InputItem, InputParam, Item, Role,
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEventKind};
use crossterm::terminal;

use crate::command::CommandHandler;
use crate::event::service::{ResponseEventService, TickEventService};
use crate::state::message::{Message, MessageSource};
use crate::state::{Mode, ResponseStage, State};

pub struct TerminalEventHandler {
    resp_service: ResponseEventService,
    tick_service: TickEventService,
}

impl TerminalEventHandler {
    pub fn new(resp_service: ResponseEventService, tick_service: TickEventService) -> Self {
        Self {
            resp_service,
            tick_service,
        }
    }

    pub fn handle(&self, event: Event, state: &mut State) -> Result<()> {
        match event {
            Event::Key(key) => match state.mode {
                Mode::Prompt => self.handle_prompt_key(key, state),
                Mode::Command => self.handle_command_key(key, state),
                Mode::Scroll => self.handle_scroll_key(key, state)?,
            },
            Event::Mouse(mouse) => match mouse.kind {
                // Step size 3 is comfortable, step size 1 feels too sluggish.
                MouseEventKind::ScrollUp => {
                    state.stream_scroll = state.stream_scroll.saturating_sub(3);
                }

                MouseEventKind::ScrollDown => {
                    state.stream_scroll = state.stream_scroll.saturating_add(3);
                }
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
                KeyCode::Char('c') => state.mode = Mode::Command,
                // CTRL-s: Exit prompt mode, enter scroll mode.
                KeyCode::Char('s') => state.mode = Mode::Scroll,
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
                KeyCode::Enter => self.submit_prompt(state),
                KeyCode::Char(ch) => state.prompt_buffer.insert_char(ch),
                KeyCode::Delete => state.prompt_buffer.delete_char(),
                KeyCode::Backspace => state.prompt_buffer.delete_prev_char(),
                KeyCode::Left => state.prompt_buffer.move_cursor_left(),
                KeyCode::Right => state.prompt_buffer.move_cursor_right(),
                KeyCode::Up => state.prompt_buffer.move_cursor_up(),
                KeyCode::Down => state.prompt_buffer.move_cursor_down(),
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
                    state.mode = Mode::Scroll;
                }
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
                // Esc: Exit command mode, enter prompt mode.
                KeyCode::Esc => {
                    state.command_buffer.clear();
                    state.mode = Mode::Prompt;
                }
                // Enter: Submit command.
                KeyCode::Enter => self.submit_command(state),
                // Backspace: Delete prev char or back to prompt mode.
                KeyCode::Backspace => {
                    if state.command_buffer.is_empty() {
                        state.mode = Mode::Prompt;
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
                KeyCode::Char('c') => state.mode = Mode::Command,
                // CTRL-u: Scroll up half screen.
                KeyCode::Char('u') => {
                    let lines = terminal::size()?.1 / 2;
                    state.stream_scroll = state.stream_scroll.saturating_sub(lines as usize);
                }
                // CTRL-d: Scroll down half screen.
                KeyCode::Char('d') => {
                    let lines = terminal::size()?.1 / 2;
                    state.stream_scroll = state.stream_scroll.saturating_add(lines as usize);
                }
                // Ctrl-y: Scroll up 1/4 screen.
                KeyCode::Char('y') => {
                    let lines = terminal::size()?.1 / 4;
                    state.stream_scroll = state.stream_scroll.saturating_sub(lines as usize);
                }
                // Ctrl-e: Scroll down 1/4 screen.
                KeyCode::Char('e') => {
                    let lines = terminal::size()?.1 / 4;
                    state.stream_scroll = state.stream_scroll.saturating_add(lines as usize);
                }
                // Ctrl-k: Scroll up 1 line.
                KeyCode::Char('k') => {
                    state.stream_scroll = state.stream_scroll.saturating_sub(1);
                }
                // Ctrl-j: Scroll down 1 line.
                KeyCode::Char('j') => {
                    state.stream_scroll = state.stream_scroll.saturating_add(1);
                }
                _ => (),
            },
            KeyModifiers::NONE => match key.code {
                // Esc: Exit scroll mode, back to prompt mode.
                KeyCode::Esc => {
                    state.mode = Mode::Prompt;
                }
                // i: Exit scroll mode, back to prompt mode.
                KeyCode::Char('i') => {
                    state.mode = Mode::Prompt;
                }
                // Up: Scroll up one line.
                KeyCode::Up => {
                    state.stream_scroll = state.stream_scroll.saturating_sub(1);
                }
                // Down: Scroll down one line.
                KeyCode::Down => {
                    state.stream_scroll = state.stream_scroll.saturating_add(1);
                }
                _ => (),
            },
            _ => (),
        };

        Ok(())
    }

    fn submit_prompt(&self, state: &mut State) {
        if matches!(state.resp_stage, ResponseStage::Finished)
            && let Some(prompt) = state.prompt_buffer.take_content()
        {
            state
                .message_store
                .commit(Message::new(MessageSource::Role(Role::User), prompt));

            state.message_store.stage(Message::new(
                MessageSource::Role(Role::Assistant),
                String::new(),
            ));

            self.resp_service.create_responses(
                make_input(state),
                state.tools.clone(),
                state.model().to_string(),
            );

            state.resp_stage = ResponseStage::Created;

            self.tick_service.run();
        }
    }

    fn submit_command(&self, state: &mut State) {
        if let Some(command) = state.command_buffer.take_content() {
            CommandHandler::handle(command, state);
        }
    }
}

fn make_input(state: &State) -> InputParam {
    let input_items = state
        .message_store
        .commits
        .iter()
        .map(|message| match &message.source {
            MessageSource::Role(role) => InputItem::EasyMessage(
                // Message build error implies code logic error and therefore is unrecoverable.
                EasyInputMessageArgs::default()
                    .role(*role)
                    .content(EasyInputContent::Text(message.content.clone()))
                    .build()
                    .expect("role and content should be enough to make EasyInputMessage."),
            ),
            MessageSource::Tool { call_id } => {
                InputItem::Item(Item::FunctionCallOutput(FunctionCallOutputItemParam {
                    call_id: call_id.clone(),
                    output: FunctionCallOutput::Text(message.content.clone()),
                    id: None,
                    status: None,
                }))
            }
        })
        .collect();

    InputParam::Items(input_items)
}
