pub mod buffer;
mod exchange;

use buffer::TextBuffer;
use exchange::Exchange;

#[derive(Default)]
pub struct State {
    exchanges: Vec<Exchange>,
    prompt_buffer: TextBuffer,
    command_buffer: TextBuffer,
    scroll_offset: usize,
}

impl State {
    pub fn set_scroll_offset(&mut self, scroll_offset: usize) {
        self.scroll_offset = scroll_offset;
    }
}

impl State {
    pub fn prompt_buffer_mut(&mut self) -> &mut TextBuffer {
        &mut self.prompt_buffer
    }

    pub fn command_buffer_mut(&mut self) -> &mut TextBuffer {
        &mut self.command_buffer
    }

    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }
}
