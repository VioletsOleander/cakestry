use super::client::Client;

mod event;
mod render;
mod state;

use state::Cursor;
use state::Exchange;
use state::UserInput;

pub struct Session {
    user_input: UserInput,
    cursor: Cursor,
    exchanges: Vec<Exchange>,

    /// The view's start index in y coordinate in the document
    view_start: usize,
    /// The view's end index in y coordinate in the document
    view_end: usize,
    /// Offset in y coordinate in the document
    offset: usize,

    client: Client,
}

impl Session {
    pub fn new(client: Client) -> Self {
        Session {
            user_input: UserInput::default(),
            cursor: Cursor::default(),
            exchanges: Vec::new(),
            view_start: 0,
            view_end: 0,
            offset: 0,
            client,
        }
    }
}
