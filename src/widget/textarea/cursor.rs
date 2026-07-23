#[derive(Default)]
pub struct Cursor {
    /// Horizontal position, byte index
    byte_idx: usize,
    /// Vertical position, line index.
    line_idx: usize,
}

impl Cursor {
    pub fn move_right(&mut self, ch: char) {
        self.byte_idx += ch.len_utf8();
    }

    pub fn move_left(&mut self, ch: char) {
        self.byte_idx -= ch.len_utf8();
    }

    pub fn jump(&mut self, byte_idx: usize, line_idx: usize) {
        self.byte_idx = byte_idx;
        self.line_idx = line_idx;
    }

    /// Return `(line index, byte index)`.
    pub fn position(&self) -> (usize, usize) {
        (self.line_idx, self.byte_idx)
    }
}
