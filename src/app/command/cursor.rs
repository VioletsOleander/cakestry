/// Cursor position on the command line.
#[derive(Default)]
pub struct Cursor {
    byte_idx: usize,
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

    /// Set cursor position to `byte_idx`.
    pub fn jump(&mut self, byte_idx: usize) {
        self.byte_idx = byte_idx;
    }

    /// Return the byte idx.
    pub fn byte_idx(&self) -> usize {
        self.byte_idx
    }
}
