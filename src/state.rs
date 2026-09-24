mod buffer;
mod exchange;

use buffer::TextBuffer;
use exchange::Exchange;

pub struct State {
    exchanges: Vec<Exchange>,
    prompt_buffer: TextBuffer,
    command_buffer: TextBuffer,
    scroll_offset: usize,
}

impl State {
    pub fn prompt_buffer_mut(&mut self) -> &mut TextBuffer {
        &mut self.prompt_buffer
    }

    pub fn command_buffer_mut(&mut self) -> &mut TextBuffer {
        &mut self.command_buffer
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            exchanges: Vec::default(),
            prompt_buffer: TextBuffer::default(),
            command_buffer: TextBuffer::default(),
            scroll_offset: 0,
        }
    }
}
