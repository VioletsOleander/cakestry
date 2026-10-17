use anyhow::{Result, anyhow};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Widget as RatatuiWidget};

use super::Widget;
use crate::tui::chatstream::textwrap::wrap_line;

pub struct Response<'a> {
    lines: Vec<&'a str>,
    scroll: u16,
}

impl<'a> Response<'a> {
    pub fn build(response: &'a str, width: usize) -> Result<Self> {
        // Textwrap algorithm expects positive width.
        if width > 0 {
            Ok(Self {
                lines: response
                    .lines()
                    .flat_map(|line| wrap_line(line, width))
                    .collect(),
                scroll: 0,
            })
        } else {
            Err(anyhow!(
                "expected to construct response view with width at least 1"
            ))
        }
    }
}

impl<'a> Widget for Response<'a> {
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
        Paragraph::new(self.lines.as_slice())
            .scroll((self.scroll, 0))
            .render(area, buf);
    }
}
