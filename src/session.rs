mod event;
pub mod state;

use state::Cursor;
use state::Exchange;
use state::UserInput;

#[derive(Default)]
pub struct Session {
    user_input: UserInput,
    cursor: Cursor,
    exchanges: Vec<Exchange>,
    scroll: usize,
}

impl Session {
    pub fn user_input(&self) -> &UserInput {
        &self.user_input
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

    pub fn last_exchange_mut(&mut self) -> &mut Exchange {
        self.exchanges
            .last_mut()
            .expect("Exchanges should be non-empty")
    }

    /// Clear and return the content of current user input.
    ///
    /// If current user input is empty, return `None`.
    pub fn take_user_input(&mut self) -> Option<String> {
        if self.user_input.is_empty() {
            return None;
        }

        let user_input = self.user_input.take_lines().join("\n");

        self.exchanges
            .push(Exchange::new(user_input.clone(), String::new()));
        self.cursor.jump(0, 0);

        Some(user_input)
    }
}
