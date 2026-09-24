pub struct TextBuffer {
    lines: Vec<String>,
    /// Index of current line.
    cursor_line: usize,
    /// Index of start byte of current char.
    cursor_char: usize,
}

impl TextBuffer {
    /// Move the cursor right by one character.
    pub fn move_cursor_right(&mut self) {
        match self.lines[self.cursor_line][self.cursor_char..]
            .chars()
            .next()
        {
            Some(ch) => self.cursor_char += ch.len_utf8(),
            None => (),
        }
    }

    /// Move the cursor left by one character.
    pub fn move_cursor_left(&mut self) {
        match self.lines[self.cursor_line][..self.cursor_char]
            .chars()
            .rev()
            .next()
        {
            Some(ch) => self.cursor_char -= ch.len_utf8(),
            None => (),
        }
    }

    pub fn set_cursor(&mut self, cursor_line: usize, cursor_char: usize) {
        self.cursor_line = cursor_line;
        self.cursor_char = cursor_char;
    }
}

impl TextBuffer {
    pub fn is_empty(&self) -> bool {
        self.lines.len() == 1 && self.lines[0].len() == 0
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        TextBuffer {
            // At least keep one empty line, zero line is not allowed.
            lines: vec![String::new()],
            cursor_line: 0,
            cursor_char: 0,
        }
    }
}
