use crate::widget::{HeightMeasurable, Renderable, Scrollable};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget};

/// A widget to display assistant reply.
pub struct Reply<'a> {
    lines: &'a [String],
    /// Scroll offset in y coordinate
    offset: u16,
}

impl<'a> Reply<'a> {
    pub fn new(lines: &'a [String]) -> Self {
        Reply {
            lines: lines,
            offset: 0,
        }
    }
}

impl<'a> Scrollable for Reply<'a> {
    fn scroll(&mut self, offset: u16) {
        self.offset = offset;
    }
}

impl<'a> HeightMeasurable for Reply<'a> {
    fn height(&self, width: u16) -> usize {
        let wrapped_lines = self
            .lines
            .iter()
            .flat_map(|line| textwrap::wrap(line, width as usize));

        wrapped_lines.count()
    }
}

impl<'a> Renderable for Reply<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let wrapped_lines: Vec<_> = self
            .lines
            .iter()
            .flat_map(|line| textwrap::wrap(line, area.width as usize))
            .map(Line::from)
            .collect();

        Paragraph::new(wrapped_lines)
            .scroll((self.offset, 0))
            .render(area, buf);
    }
}
