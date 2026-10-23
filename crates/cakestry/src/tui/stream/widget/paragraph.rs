use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph as RatatuiParagraph, Widget};

use super::Scroll;

pub struct Paragraph<'a> {
    lines: Vec<&'a str>,
    scroll: u16,
}

impl<'a> Paragraph<'a> {
    pub fn new(lines: Vec<&'a str>) -> Self {
        Self { lines, scroll: 0 }
    }
}

impl<'a> Widget for Paragraph<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        RatatuiParagraph::new(self.lines.as_slice())
            .scroll((self.scroll, 0))
            .render(area, buf);
    }
}

impl<'a> Scroll for Paragraph<'a> {
    fn scroll(self, scroll: u16) -> Self {
        Self {
            lines: self.lines,
            scroll,
        }
    }
}
