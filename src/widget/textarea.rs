use crate::widget::{HeightMeasurable, Scrollable};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

mod cursor;
mod lines;

use cursor::Cursor;
use lines::Lines;

pub struct TextArea {
    // The length is at least 1
    lines: Lines,
    cursor: Cursor,
    // Scroll offset in y coordinate
    offset: u16,
}

impl Default for TextArea {
    fn default() -> Self {
        TextArea {
            lines: Lines::from(vec![String::new()]),
            cursor: Cursor::default(),
            offset: 0,
        }
    }
}

impl TextArea {
    pub fn handle_key(&mut self, key: KeyEvent) {
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

        self.lines.insert_char(line_idx, byte_idx, ch);
        self.cursor.move_right(ch);
    }

    fn remove_char(&mut self) {
        let (line_idx, byte_idx) = self.cursor.position();

        // Delete current line's '\n' char
        // Equivalent to joining current line and next line
        if byte_idx == self.lines[line_idx].len() {
            if line_idx == self.lines.len() {
                return;
            }

            let next_line = self.lines.remove_line(line_idx + 1);
            self.lines.insert_str(line_idx, byte_idx, &next_line);
            return;
        }

        self.lines.remove_char(line_idx, byte_idx);
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
            let prev_line_len = self.lines[prev_line_idx].len();
            let curr_line = self.lines.remove_line(prev_line_idx);

            self.lines
                .insert_str(prev_line_idx, prev_line_len, &curr_line);
            self.cursor.jump(prev_line_len, prev_line_idx);

            return;
        }

        // Find previous char's byte index
        let bytes = self.lines[line_idx].as_bytes();

        // UTF-8 contiuation bytes all has the form of 0b10xx_xxxx
        // By UTF-8's design, the number of bytes to go back is at most 3
        while (bytes[byte_idx] & 0b1100_0000) == 0b100_0000 {
            byte_idx -= 1;
        }

        let ch = self.lines.remove_char(line_idx, byte_idx);
        self.cursor.move_left(ch);
    }

    fn break_line(&mut self) {
        let (line_idx, byte_idx) = self.cursor.position();

        let next_line_idx = line_idx + 1;
        let next_line = String::from(&self.lines[line_idx][byte_idx..]);

        self.lines.truncate_line(line_idx, byte_idx);
        self.lines.insert_line(next_line_idx, next_line);
        self.cursor.jump(0, next_line_idx);
    }
}

impl Widget for &TextArea {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
    }
}
