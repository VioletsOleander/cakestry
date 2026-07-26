/// Cursor on the screen.
#[derive(Default)]
pub struct Cursor {
    /// Horizontal position, byte index
    byte_idx: usize,
    /// Vertical position, line index.
    line_idx: usize,
}

impl Cursor {
    /// Move the cursor position right by `ch`.
    pub fn move_right(&mut self, ch: char) {
        self.byte_idx += ch.len_utf8();
    }

    /// Move the cursor position left by `ch`.
    pub fn move_left(&mut self, ch: char) {
        self.byte_idx -= ch.len_utf8();
    }

    /// Set cursor poisition to `(line_idx, byte_idx)`.
    pub fn jump(&mut self, line_idx: usize, byte_idx: usize) {
        self.line_idx = line_idx;
        self.byte_idx = byte_idx;
    }

    /// Return `(line index, byte index)`.
    pub fn position(&self) -> (usize, usize) {
        (self.line_idx, self.byte_idx)
    }
}
