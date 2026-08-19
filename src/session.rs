mod event;
pub mod state;

use state::Cursor;
use state::Exchange;
use state::UserInput;

#[derive(Default)]
pub struct Session {
    user_input: UserInput,
    agent_output: Option<String>,
    cursor: Cursor,
    exchanges: Vec<Exchange>,
    scroll: usize,
}

impl Session {
    pub fn user_input(&self) -> &UserInput {
        &self.user_input
    }

    pub fn agent_output(&self) -> &Option<String> {
        &self.agent_output
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn exchanges(&self) -> &Vec<Exchange> {
        &self.exchanges
    }

    pub fn scroll(&self) -> usize {
        self.scroll
    }
}
