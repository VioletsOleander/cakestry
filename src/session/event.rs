use crate::session::Session;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

impl Session {
    /// Handle event and modify internal state correspondingly.
    pub fn handle_event(&mut self, event: Event) {
        match event {
            // Event::Key(key) => match key.code {
            //     KeyCode::Enter => {
            //         // if self.textarea.is_empty() {
            //         //     return;
            //         // }
            //         //
            //         // self.messsages
            //         //     .push(Message::new(self.textarea.lines().join("\n"), true));
            //         // self.textarea.clear();
            //     }
            //     _ => {
            //         self.handle_key(key);
            //     }
            // },
            Event::Key(key) => {
                self.handle_key(key);
            }
            Event::Mouse(mouse) => {
                self.handle_mouse(mouse);
            }
            _ => (),
        }
    }
}

impl Session {
    fn handle_key(&mut self, key: KeyEvent) {
        match key.modifiers {
            KeyModifiers::CONTROL => match key.code {
                // This also matches Ctrl-Enter
                KeyCode::Char('j') => {
                    self.break_line();
                }
                _ => (),
            },
            KeyModifiers::SHIFT => match key.code {
                KeyCode::Char(ch) => {
                    self.insert_char(ch);
                }
                _ => (),
            },
            KeyModifiers::NONE => match key.code {
                KeyCode::Delete => {
                    self.remove_char();
                }
                KeyCode::Backspace => {
                    self.remove_prev_char();
                }
                KeyCode::Char(ch) => {
                    self.insert_char(ch);
                }
                _ => (),
            },
            _ => (),
        }
    }

    fn insert_char(&mut self, ch: char) {
        let (line_idx, byte_idx) = self.cursor.position();

        self.user_input.insert_char(line_idx, byte_idx, ch);
        self.cursor.move_right(ch);
    }

    fn remove_char(&mut self) {
        let (line_idx, byte_idx) = self.cursor.position();

        // Delete current line's '\n' char
        // Equivalent to joining current line and next line
        if byte_idx == self.user_input[line_idx].len() {
            if line_idx == self.user_input.len() {
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

        // Delete previous line's '\n' char
        // Equivalent to joining previous line and current line
        if byte_idx == 0 {
            if line_idx == 0 {
                return;
            }

            let prev_line_idx = line_idx - 1;
            let prev_line_len = self.user_input[prev_line_idx].len();
            let curr_line = self.user_input.remove_line(prev_line_idx);

            self.user_input
                .insert_str(prev_line_idx, dbg!(prev_line_len), &curr_line);
            self.cursor.jump(prev_line_idx, prev_line_len);

            return;
        }

        // Find previous char's byte index
        let bytes = self.user_input[line_idx].as_bytes();

        // UTF-8 contiuation bytes all has the form of 0b10xx_xxxx
        // By UTF-8's design, the number of bytes to go back is at most 3
        loop {
            byte_idx -= 1;
            if (bytes[byte_idx] & 0b1100_0000) != 0b1000_0000 {
                break;
            }
        }

        let ch = self.user_input.remove_char(line_idx, byte_idx);
        self.cursor.move_left(ch);
    }

    fn break_line(&mut self) {
        let (line_idx, byte_idx) = self.cursor.position();

        let next_line_idx = line_idx + 1;
        let next_line = String::from(&self.user_input[line_idx][byte_idx..]);

        self.user_input.truncate_line(line_idx, byte_idx);
        self.user_input.insert_line(next_line_idx, next_line);
        self.cursor.jump(next_line_idx, 0);
    }
}

impl Session {
    fn handle_mouse(&mut self, mouse: MouseEvent) {
        match mouse.kind {
            MouseEventKind::ScrollDown => {
                self.view_start = self.view_start.saturating_add(1);
            }
            MouseEventKind::ScrollUp => {
                self.view_start = self.view_start.saturating_sub(1);
            }
            _ => (),
        }
    }
}
