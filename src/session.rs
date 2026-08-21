mod event;
pub mod state;

use state::Cursor;
use state::Exchange;
use state::UserInput;

#[derive(Default)]
pub struct Session {
    exchanges: Vec<Exchange>,
    user_input: UserInput,
    agent_output: Option<String>,
    cursor: Cursor,
    scroll: usize,
}

impl Session {
    pub fn exchanges(&self) -> &Vec<Exchange> {
        &self.exchanges
    }

    pub fn user_input(&self) -> &UserInput {
        &self.user_input
    }

    pub fn agent_output(&self) -> &Option<String> {
        &self.agent_output
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn scroll(&self) -> usize {
        self.scroll
    }

    pub fn user_input_mut(&mut self) -> &mut UserInput {
        &mut self.user_input
    }

    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    pub fn set_agent_output(&mut self, agent_output: Option<String>) {
        self.agent_output = agent_output;
    }
}
