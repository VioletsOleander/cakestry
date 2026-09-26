pub enum TextBufferKind {
    Prompt,
    Command,
}

pub struct TextBuffer {
    lines: Vec<String>,
    /// Index of current line.
    cursor_line: usize,
    /// Index of start byte of current char.
    cursor_char: usize,
}

// Regular keyboard actions.
impl TextBuffer {
    /// Insert `ch` at cursor.
    pub fn insert_char(&mut self, ch: char) {
        self.lines[self.cursor_line].insert(self.cursor_char, ch);
        self.cursor_char += ch.len_utf8();
    }

    /// Delete the character at cursor.
    pub fn delete_char(&mut self) {
        // If in the line end, join the next line.
        if self.cursor_char == self.lines[self.cursor_line].len() {
            // If in the last line, nothing to join.
            if self.cursor_line == self.lines.len() - 1 {
                return;
            }

            let next_line = self.lines.remove(self.cursor_line + 1);
            self.lines[self.cursor_line].push_str(&next_line);
        } else {
            self.lines[self.cursor_line].remove(self.cursor_char);
        }
    }

    /// Delete the character before cursor.
    pub fn delete_prev_char(&mut self) {
        // If in the line start, join to the previous line.
        if self.cursor_char == 0 {
            // If in the first line, nothing to join to.
            if self.cursor_line == 0 {
                return;
            }

            let prev_cursor_line = self.cursor_line - 1;
            let prev_cursor_char = self.lines[prev_cursor_line].len();
            let curr_line = self.lines.remove(self.cursor_line);

            self.lines[prev_cursor_line].push_str(&curr_line);
            self.cursor_line = prev_cursor_line;
            self.cursor_char = prev_cursor_char;
        } else {
            let bytes = self.lines[self.cursor_line].as_bytes();

            // The number of backward steps should at most be 3 by the design of UTF-8.
            loop {
                self.cursor_char -= 1;

                // UTF-8 continuation bytes ranges in [0x80, 0xBF], i.e. [1000_0000, 1011_1111].
                // Intepreted as i8, continuation bytes' range is [-128, -65],
                // and boundary bytes' range is [-64 (-0x40), 128].
                // Therefore this operation is equvalent to (byte < 0x7F || byte >= 0xC0).
                if (bytes[self.cursor_char] as i8) >= -0x40 {
                    break;
                }
            }

            self.lines[self.cursor_line].remove(self.cursor_char);
        }
    }
}

// Regular mouse actions.
impl TextBuffer {
    /// Move the cursor left by one character.
    pub fn move_cursor_left(&mut self) {
        if let Some(ch) = self.lines[self.cursor_line][..self.cursor_char]
            .chars()
            .next_back()
        {
            self.cursor_char -= ch.len_utf8()
        }
    }

    /// Move the cursor right by one character.
    pub fn move_cursor_right(&mut self) {
        if let Some(ch) = self.lines[self.cursor_line][self.cursor_char..]
            .chars()
            .next()
        {
            self.cursor_char += ch.len_utf8()
        }
    }
}

// Special keyboard actions.
impl TextBuffer {
    /// Break the line at cursor.
    ///
    /// The behaviour aligns with i_<CR>/i_CTRL-M in vim.
    pub fn break_line(&mut self) {
        let suffix = self.lines[self.cursor_line].split_off(self.cursor_char);

        self.cursor_line += 1;
        self.cursor_char = 0;
        self.lines.insert(self.cursor_line, suffix);
    }

    /// Delete the word (prefix) before the cursor.
    ///
    /// The behaviour aligns with i_CTRL-W in vim.
    pub fn delete_word_backward(&mut self) {
        let bytes = self.lines[self.cursor_line].as_bytes();

        // First try finding left nearest non-whitespace character, i.e. last word's end.
        let mut idx = self.cursor_char;
        while idx > 0 {
            idx -= 1;
            if !matches!(bytes[idx], b'\t' | b' ') {
                break;
            }
        }

        // Next try finding left nearest whitspace character, i.e. last word's begin.
        while idx > 0 {
            idx -= 1;
            if matches!(bytes[idx], b'\t' | b' ') {
                // The delete range starts from the character after the whitspace.
                idx += 1;
                break;
            }
        }

        self.lines[self.cursor_line].drain(idx..self.cursor_char);
        self.cursor_char = idx;
    }

    /// Delete the line (prefix) before the cursor.
    ///
    /// Thie behaviour aligns with i_CTRL-U in vim.
    pub fn delete_line_backward(&mut self) {
        self.lines[self.cursor_char].drain(0..self.cursor_char);
        self.cursor_char = 0;
    }
}

impl TextBuffer {
    pub fn is_empty(&self) -> bool {
        self.lines.len() == 1 && self.lines[0].len() == 0
    }
}

// Getters.
impl TextBuffer {
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn cursor_line(&self) -> usize {
        self.cursor_line
    }

    pub fn cursor_char(&self) -> usize {
        self.cursor_char
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn delete_char() {
        let line = "Hello world";
        let expected_line = "Hello worl";
        let mut buffer = TextBuffer {
            lines: Vec::from([String::from(line)]),
            cursor_line: 0,
            cursor_char: expected_line.len(),
        };

        buffer.delete_char();

        assert_eq!(buffer.lines.len(), 1);
        assert_eq!(buffer.lines[0], expected_line);
        assert_eq!(buffer.cursor_line, 0);
        assert_eq!(buffer.cursor_char, expected_line.len());
    }

    #[test]
    fn delete_char_line_end() {
        let line0 = "Hello wo";
        let line1 = "rld";
        let mut buffer = TextBuffer {
            lines: Vec::from([String::from(line0), String::from(line1)]),
            cursor_line: 0,
            cursor_char: line0.len(),
        };

        buffer.delete_char();

        assert_eq!(buffer.lines.len(), 1);
        assert_eq!(buffer.lines[0], "Hello world");
        assert_eq!(buffer.cursor_line, 0);
        assert_eq!(buffer.cursor_char, line0.len());
    }

    #[test]
    fn delete_prev_char() {
        let line = "Hello world";
        let expected_line = "Hello worl";
        let mut buffer = TextBuffer {
            lines: Vec::from([String::from(line)]),
            cursor_line: 0,
            cursor_char: line.len(),
        };

        buffer.delete_prev_char();

        assert_eq!(buffer.lines.len(), 1);
        assert_eq!(buffer.lines[0], expected_line);
        assert_eq!(buffer.cursor_line, 0);
        assert_eq!(buffer.cursor_char, expected_line.len());
    }

    #[test]
    fn delete_prev_char_line_start() {
        let line0 = "Hello wo";
        let line1 = "rld";
        let mut buffer = TextBuffer {
            lines: Vec::from([String::from(line0), String::from(line1)]),
            cursor_line: 1,
            cursor_char: 0,
        };

        buffer.delete_prev_char();

        assert_eq!(buffer.lines.len(), 1);
        assert_eq!(buffer.lines[0], "Hello world");
        assert_eq!(buffer.cursor_line, 0);
        assert_eq!(buffer.cursor_char, line0.len());
    }

    #[test]
    fn move_cursor_left() {
        let line = "Hello world";
        let mut buffer = TextBuffer {
            lines: Vec::from([String::from(line)]),
            cursor_line: 0,
            cursor_char: line.len(),
        };

        buffer.move_cursor_left();

        assert_eq!(buffer.cursor_line, 0);
        assert_eq!(buffer.cursor_char, "Hello worl".len());
    }

    #[test]
    fn move_cursor_left_line_start() {
        let line = "Hello world";
        let mut buffer = TextBuffer {
            lines: Vec::from([String::from(line)]),
            cursor_line: 0,
            cursor_char: 0,
        };

        buffer.move_cursor_left();

        assert_eq!(buffer.cursor_line, 0);
        assert_eq!(buffer.cursor_char, 0);
    }

    #[test]
    fn move_cursor_right() {
        let line = "Hello world";
        let mut buffer = TextBuffer {
            lines: Vec::from([String::from(line)]),
            cursor_line: 0,
            cursor_char: 0,
        };

        buffer.move_cursor_right();

        assert_eq!(buffer.cursor_line, 0);
        assert_eq!(buffer.cursor_char, "H".len());
    }

    #[test]
    fn move_cursor_right_line_end() {
        let line = "Hello world";
        let mut buffer = TextBuffer {
            lines: Vec::from([String::from(line)]),
            cursor_line: 0,
            cursor_char: line.len(),
        };

        buffer.move_cursor_right();

        assert_eq!(buffer.cursor_line, 0);
        assert_eq!(buffer.cursor_char, line.len());
    }

    #[test]
    fn break_line() {
        let line = "Hello world";
        let mut buffer = TextBuffer {
            lines: Vec::from([String::from(line)]),
            cursor_line: 0,
            cursor_char: "Hello worl".len(),
        };

        buffer.break_line();

        assert_eq!(buffer.lines.len(), 2);
        assert_eq!(buffer.lines[0], "Hello worl");
        assert_eq!(buffer.lines[1], "d");
        assert_eq!(buffer.cursor_line, 1);
        assert_eq!(buffer.cursor_char, 0);
    }

    #[test]
    fn delete_word_backward() {
        let line = "Hello world";
        let mut buffer = TextBuffer {
            lines: Vec::from([String::from(line)]),
            cursor_line: 0,
            cursor_char: line.len(),
        };

        buffer.delete_word_backward();

        assert_eq!(buffer.lines.len(), 1);
        assert_eq!(buffer.lines[0], "Hello ");
        assert_eq!(buffer.cursor_line, 0);
        assert_eq!(buffer.cursor_char, "Hello ".len());
    }

    #[test]
    fn delete_word_backward_inside_word() {
        let line = "Hello world";
        let mut buffer = TextBuffer {
            lines: Vec::from([String::from(line)]),
            cursor_line: 0,
            cursor_char: "Hello worl".len(),
        };

        buffer.delete_word_backward();

        assert_eq!(buffer.lines.len(), 1);
        assert_eq!(buffer.lines[0], "Hello d");
        assert_eq!(buffer.cursor_line, 0);
        assert_eq!(buffer.cursor_char, "Hello ".len());
    }

    #[test]
    fn delete_word_backward_outside_word() {
        let line = "Hello world";
        let mut buffer = TextBuffer {
            lines: Vec::from([String::from(line)]),
            cursor_line: 0,
            cursor_char: "Hello ".len(),
        };

        buffer.delete_word_backward();

        assert_eq!(buffer.lines.len(), 1);
        assert_eq!(buffer.lines[0], "world");
        assert_eq!(buffer.cursor_line, 0);
        assert_eq!(buffer.cursor_char, 0);
    }

    #[test]
    fn delete_word_backward_without_prefix_whitespace() {
        let line = "Hello";
        let mut buffer = TextBuffer {
            lines: Vec::from([String::from(line)]),
            cursor_line: 0,
            cursor_char: line.len(),
        };

        buffer.delete_word_backward();

        assert_eq!(buffer.lines.len(), 1);
        assert_eq!(buffer.lines[0], "");
        assert_eq!(buffer.cursor_line, 0);
        assert_eq!(buffer.cursor_char, 0);
    }

    #[test]
    fn delete_word_backward_with_prefix_whitspace() {
        let line = " Hello";
        let mut buffer = TextBuffer {
            lines: Vec::from([String::from(line)]),
            cursor_line: 0,
            cursor_char: line.len(),
        };

        buffer.delete_word_backward();

        assert_eq!(buffer.lines.len(), 1);
        assert_eq!(buffer.lines[0], " ");
        assert_eq!(buffer.cursor_line, 0);
        assert_eq!(buffer.cursor_char, 1);
    }
}
