use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Widget as RatatuiWidget};

use super::{ReservedWidth, Widget};

/// A widget to display assistant reply.
#[derive(Debug)]
pub struct Reply<'a> {
    wrapped_lines: Vec<&'a str>,
    /// Scroll offset in y coordinate
    offset: u16,
}

impl<'a> Reply<'a> {
    /// Create a `Reply` from given lines.
    pub fn new(wrapped_lines: Vec<&'a str>) -> Self {
        Reply {
            wrapped_lines,
            offset: 0,
        }
    }
}

impl<'a> Widget for Reply<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(self.wrapped_lines.as_slice())
            .scroll((self.offset, 0))
            .render(area, buf);
    }

    fn height(&self) -> usize {
        self.wrapped_lines.len()
    }

    fn scroll(&mut self, offset: u16) {
        self.offset = offset;
    }
}

impl<'a> ReservedWidth for Reply<'a> {
    fn reserved_width() -> usize {
        0
    }
}
