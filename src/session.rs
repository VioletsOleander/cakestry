use super::client::Client;

mod event;
pub mod state;

use state::Cursor;
use state::Exchange;
use state::UserInput;

// Data structure stores and modifies its data, not responsible for rendering itself.
pub struct Session {
    user_input: UserInput,
    cursor: Cursor,
    exchanges: Vec<Exchange>,
    scroll: usize,
    client: Client,
}

impl Session {
    pub fn new(client: Client) -> Self {
        Session {
            user_input: UserInput::default(),
            cursor: Cursor::default(),
            exchanges: Vec::new(),
            scroll: 0,
            client,
        }
    }

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

    pub fn client(&self) -> &Client {
        &self.client
    }
}
