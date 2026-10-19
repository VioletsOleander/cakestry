use anyhow::{Result, anyhow};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph as RatatuiParagraph, Widget as RatatuiWidget};

use super::Widget;
use crate::tui::stream::textwrap::wrap_line;

pub struct Paragraph<'a> {
    lines: Vec<&'a str>,
    scroll: u16,
}

impl<'a> Paragraph<'a> {
    pub fn build(content: &'a str, width: usize) -> Result<Self> {
        // Textwrap algorithm expects positive width.
        if width > 0 {
            Ok(Self {
                lines: content
                    .lines()
                    .flat_map(|line| wrap_line(line, width))
                    .collect(),
                scroll: 0,
            })
        } else {
            Err(anyhow!(
                "expected to construct paragraph with width at least 1"
            ))
        }
    }
}

impl<'a> Widget for Paragraph<'a> {
    fn scroll(self, scroll: u16) -> Self {
        Self {
            lines: self.lines,
            scroll,
        }
    }

    fn height(&self) -> usize {
        self.lines.len()
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        RatatuiParagraph::new(self.lines.as_slice())
            .scroll((self.scroll, 0))
            .render(area, buf);
    }
}
