use super::{ReservedWidth, Widget};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Paragraph, Widget as RatatuiWidget};
use std::borrow::Cow;

mod angle;
mod dot;

use angle::Angle;
use dot::Dot;

/// A widget to display wrapped user input text.
pub struct Query<'a> {
    wrapped_lines: Vec<Cow<'a, str>>,
    /// Scroll offset in y coordinate
    offset: u16,
}

impl<'a> Query<'a> {
    pub fn new(wrapped_lines: Vec<Cow<'a, str>>) -> Self {
        Query {
            wrapped_lines,
            offset: 0,
        }
    }
}

impl<'a> Widget for Query<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [spans_area, paragraph_area] =
            Layout::horizontal([Constraint::Length(1), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);

        for i in 0..spans_area.height {
            if i < self.offset {
                continue;
            }

            let span_area = Rect::new(spans_area.x, spans_area.y + i, spans_area.width, 1);
            if i == 0 {
                Angle.render(span_area, buf);
            } else {
                Dot.render(span_area, buf);
            }
        }

        Paragraph::new(self.wrapped_lines.as_slice())
            .scroll((self.offset, 0))
            .render(paragraph_area, buf);
    }

    fn height(&self) -> usize {
        self.wrapped_lines.len()
    }

    fn scroll(&mut self, offset: u16) {
        self.offset = offset;
    }
}

impl<'a> ReservedWidth for Query<'a> {
    fn reserved_width() -> usize {
        // 1 width span + 1 width space
        2
    }
}
