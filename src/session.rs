mod event;
mod render;
mod state;

use state::Cursor;
use state::Exchange;
use state::UserInput;

#[derive(Default)]
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
}
