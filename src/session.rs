mod event;
pub mod state;

use state::Cursor;
pub use state::Exchange;
use state::UserInput;

#[derive(Default)]
pub struct Session {
    exchanges: Vec<Exchange>,
    user_input: UserInput,
    cursor: Cursor,
    scroll: usize,
}

impl Session {
    /// Clear user input and return the its content.
    pub fn take_user_input(&mut self) -> String {
        let user_input = self.user_input.lines().join("\n");

        self.user_input.clear();
        self.cursor.jump(0, 0);

        user_input
    }
}

impl Session {
    pub fn exchanges(&self) -> &Vec<Exchange> {
        &self.exchanges
    }

    pub fn user_input(&self) -> &UserInput {
        &self.user_input
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn scroll(&self) -> usize {
        self.scroll
    }

    pub fn exchanges_mut(&mut self) -> &mut Vec<Exchange> {
        &mut self.exchanges
    }
}
