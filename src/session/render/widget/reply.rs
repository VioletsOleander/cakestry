use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Widget as RatatuiWidget};

use super::Widget;
use crate::session::render::wrap::wrap_line;

/// A widget to display assistant reply.
#[derive(Debug)]
pub struct Reply<'a> {
    /// Lines of text wrapped by the specified width.
    lines: Vec<&'a str>,
    /// Scroll offset in y coordinate.
    offset: u16,
}

impl<'a> Reply<'a> {
    /// Create a `Reply` from given `lines` and `width`.
    ///
    /// `width` is used to appropriately wrap `lines`.
    pub fn new(lines: Vec<&'a str>, width: usize) -> Self {
        let wrapped_lines = lines
            .iter()
            .flat_map(|line| wrap_line(line, width))
            .collect();

        Reply {
            lines: wrapped_lines,
            offset: 0,
        }
    }
}

impl<'a> Widget for Reply<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(self.lines.as_slice())
            .scroll((self.offset, 0))
            .render(area, buf);
    }

    fn height(&self) -> usize {
        self.lines.len()
    }

    fn scroll(&mut self, offset: u16) {
        self.offset = offset;
    }
}
