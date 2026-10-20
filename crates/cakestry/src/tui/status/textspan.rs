use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::text::Span;
use ratatui::widgets::Widget as RatatuiWidget;
use unicode_width::UnicodeWidthStr;

pub struct TextSpan<'a> {
    content: &'a str,
    cursor: usize,
}

impl<'a> TextSpan<'a> {
    pub fn new(content: &'a str, cursor: usize) -> Self {
        Self {
            content,
            cursor: content[..cursor].width(),
        }
    }

    pub fn compute_cursor_position(&self, area: Rect) -> Option<Position> {
        let x_offset = self.cursor as u16 + 1; // 1 for the width of prefix ":"

        if x_offset < area.width {
            Some(Position::new(area.x + x_offset, area.y))
        } else {
            None
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let [colon_area, command_area] =
            Layout::horizontal([Constraint::Length(1), Constraint::Fill(1)]).areas(area);

        Span::from(":").render(colon_area, buf);
        Span::from(self.content).render(command_area, buf);
    }
}
