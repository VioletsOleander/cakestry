use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::session::Session;
use crate::session::state::Exchange;

impl Session {
    pub(super) fn handle_key(&mut self, key: KeyEvent) {
        match key.modifiers {
            KeyModifiers::CONTROL => match key.code {
                // This also matches Ctrl-Enter
                KeyCode::Char('j') => self.break_line(),
                KeyCode::Char('w') => self.clear_to_word_begin(),
                KeyCode::Char('u') => self.clear_to_line_begin(),
                _ => (),
            },
            KeyModifiers::SHIFT => match key.code {
                KeyCode::Char(ch) => self.insert_char(ch),
                _ => (),
            },
            KeyModifiers::NONE => match key.code {
                KeyCode::Delete => self.remove_char(),
                KeyCode::Backspace => self.remove_prev_char(),
                KeyCode::Char(ch) => self.insert_char(ch),
                KeyCode::Left => self.move_cursor_left(),
                KeyCode::Right => self.move_cursor_right(),
                KeyCode::Enter => {
                    if self.user_input.is_empty() {
                        return;
                    }
                    self.send_request();
                }
                _ => (),
            },
            _ => (),
        }
    }
}

impl Session {
    fn break_line(&mut self) {
        let (line_idx, byte_idx) = self.cursor.position();

        let next_line_idx = line_idx + 1;
        let next_line = String::from(&self.user_input[line_idx][byte_idx..]);

        self.user_input.truncate_line(line_idx, byte_idx);
        self.user_input.insert_line(next_line_idx, next_line);
        self.cursor.jump(next_line_idx, 0);
    }

    fn clear_to_word_begin(&mut self) {
        let (line_idx, byte_idx) = self.cursor.position();

        // The clear range starts from the next char of the whitespace char,
        // or starts from the line begining.
        let bytes = self.user_input[line_idx].as_bytes();
        let mut start_idx = byte_idx;

        // Firstly find left nearest non-whitespace char, equivalent to find last word's end.
        while start_idx > 0 {
            start_idx -= 1;

            if !bytes[start_idx].is_ascii_whitespace() {
                break;
            }
        }

        // Then find the left nearest whitespace char, equivalent to find last word's begin.
        while start_idx > 0 {
            start_idx -= 1;

            if bytes[start_idx].is_ascii_whitespace() {
                start_idx += 1;
                break;
            }
        }

        self.user_input.drain_line(line_idx, start_idx..byte_idx);
        self.cursor.jump(line_idx, start_idx);
    }

    fn clear_to_line_begin(&mut self) {
        let (line_idx, byte_idx) = self.cursor.position();

        self.user_input.drain_line(line_idx, 0..byte_idx);
        self.cursor.jump(line_idx, 0);
    }
}

impl Session {
    async fn send_request(&mut self) {
        let query_lines = self.user_input.take_lines();
        let reply_lines = vec![String::from("reply")];

        self.exchanges.push(Exchange::new(query_lines, reply_lines));
        self.cursor.jump(0, 0);
    }

    fn insert_char(&mut self, ch: char) {
        let (line_idx, byte_idx) = self.cursor.position();

        self.user_input.insert_char(line_idx, byte_idx, ch);
        self.cursor.move_right(ch);
    }

    fn remove_char(&mut self) {
        let (line_idx, byte_idx) = self.cursor.position();

        // If current char is in the last, delete the next '\n' char,
        // which is equivalent to joining with the next line.
        if byte_idx == self.user_input[line_idx].len() {
            // If current is in the last, do nothing.
            if line_idx == self.user_input.len() - 1 {
                return;
            }

            let next_line = self.user_input.remove_line(line_idx + 1);
            self.user_input.insert_str(line_idx, byte_idx, &next_line);
            return;
        }

        self.user_input.remove_char(line_idx, byte_idx);
    }

    fn remove_prev_char(&mut self) {
        let (line_idx, mut byte_idx) = self.cursor.position();

        // If current char is in the first, delete the previous '\n' char,
        // which is equivalent to joining with the previous line.
        if byte_idx == 0 {
            // If current line is in the first, do nothing.
            if line_idx == 0 {
                return;
            }

            let prev_line_idx = line_idx - 1;
            let prev_line_len = self.user_input[prev_line_idx].len();
            let curr_line = self.user_input.remove_line(line_idx);

            self.user_input
                .insert_str(prev_line_idx, prev_line_len, &curr_line);
            self.cursor.jump(prev_line_idx, prev_line_len);

            return;
        }

        // Find previous char's starting byte index.
        let bytes = self.user_input[line_idx].as_bytes();
        loop {
            // The UTF-8 contiuation bytes all has a form of 0b10xx_xxxx.
            // The number of bytes to go back is at most 3 by the design of UTF-8.
            byte_idx -= 1;
            if (bytes[byte_idx] & 0b1100_0000) != 0b1000_0000 {
                break;
            }
        }

        let ch = self.user_input.remove_char(line_idx, byte_idx);
        self.cursor.move_left(ch);
    }

    fn move_cursor_left(&mut self) {
        let (line_idx, mut byte_idx) = self.cursor.position();

        // If there is no prev char, do nothing.
        if byte_idx == 0 {
            return;
        }

        // Find previous char's starting byte index.
        let bytes = self.user_input[line_idx].as_bytes();
        loop {
            // The UTF-8 contiuation bytes all has a form of 0b10xx_xxxx.
            // The number of bytes to go back is at most 3 by the design of UTF-8.
            byte_idx -= 1;
            if (bytes[byte_idx] & 0b1100_0000) != 0b1000_0000 {
                break;
            }
        }

        let ch = self.user_input.char(line_idx, byte_idx);
        self.cursor.move_left(ch);
    }

    fn move_cursor_right(&mut self) {
        let (line_idx, byte_idx) = self.cursor.position();

        // If there is no current char, do nothing.
        if byte_idx == self.user_input[line_idx].len() {
            return;
        }

        let ch = self.user_input.char(line_idx, byte_idx);
        self.cursor.move_right(ch);
    }
}
