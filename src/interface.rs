use crate::session::state::State;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Span;
use ratatui::widgets::{Paragraph, Widget, Wrap};
use ratatui_textarea::TextArea;

#[derive(Default)]
pub struct Interface {
    textarea: TextArea<'static>,
}

impl Interface {
    pub fn render(&self, state: &State, area: Rect, buf: &mut Buffer) {
        let mut height: u16 = 0;

        for (i, message) in state.messages().iter().enumerate() {
            if i % 2 == 0 {
                height += self.render_request(
                    message,
                    Rect::new(area.x, height, area.width, area.height),
                    buf,
                );
            } else {
                height += self.render_response(
                    message,
                    Rect::new(area.x, height, area.width, area.height),
                    buf,
                );
            }
        }

        self.render_textarea(Rect::new(area.x, height, area.width, area.height), buf);
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
            // For ctrl-j and ctrl-enter
            self.textarea.insert_newline();
        } else {
            self.textarea.input(key);
        }
    }

    /// Return the height of the rendered area.
    fn render_request(&self, request: &str, area: Rect, buf: &mut Buffer) -> u16 {
        let prompt = Span::raw(">");
        let paragraph = &Paragraph::new(request).wrap(Wrap { trim: true });

        prompt.render(Rect::new(area.x, area.y, 1, area.height), buf);
        paragraph.render(
            Rect::new(area.x + 2, area.y, area.width - 2, area.height),
            buf,
        );

        paragraph.line_count(area.width - 2) as u16
    }

    /// Return the height of the rendered area.
    fn render_response(&self, response: &str, area: Rect, buf: &mut Buffer) -> u16 {
        let paragraph = &Paragraph::new(response).wrap(Wrap { trim: true });
        paragraph.render(area, buf);

        paragraph.line_count(area.width) as u16
    }

    fn render_textarea(&self, area: Rect, buf: &mut Buffer) -> () {
        let prompt = Span::raw(">");

        prompt.render(Rect::new(area.x, area.y, 1, area.height), buf);
        self.textarea.render(
            Rect::new(area.x + 2, area.y, area.width - 2, area.height),
            buf,
        );
    }
}
