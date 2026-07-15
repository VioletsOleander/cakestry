use crate::session::state::State;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Widget, Wrap};
use ratatui_textarea::TextArea;

#[derive(Default)]
pub struct Interface {
    textarea: TextArea<'static>,
}

impl Interface {
    pub fn render(&self, state: &State, area: Rect, buf: &mut Buffer) {
        let mut height: u16 = 0;

        for message in state.messages() {
            let paragraph = &Paragraph::new(message.as_str()).wrap(Wrap { trim: true });
            paragraph.render(Rect::new(area.x, height, area.width, area.height), buf);

            height += paragraph.line_count(area.width) as u16;
        }

        self.textarea
            .render(Rect::new(area.x, height, area.width, area.height), buf);
    }

    pub fn get_input(&self) -> String {
        self.textarea.lines().join("\n")
    }

    pub fn clear_input(&mut self) -> () {
        self.textarea.clear();
    }

    pub fn pop_input(&mut self) -> String {
        let input = self.get_input();
        self.clear_input();
        input
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> () {
        if key.code == KeyCode::Char('j') && key.modifiers == KeyModifiers::CONTROL {
            self.textarea.insert_newline();
        } else {
            self.textarea.input(key);
        }
    }
}
