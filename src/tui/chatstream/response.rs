use anyhow::{Result, anyhow};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Widget};

use super::wrap_line;

pub struct ResponseView<'a> {
    lines: Vec<&'a str>,
    scroll: u16,
}

impl<'a> ResponseView<'a> {
    pub fn build(response: &'a str, width: usize) -> Result<Self> {
        // Textwrap algorithm expects positive width.
        match width > 0 {
            true => Ok(Self {
                lines: response
                    .lines()
                    .flat_map(|line| wrap_line(line, width))
                    .collect(),
                scroll: 0,
            }),
            false => Err(anyhow!(
                "expected to construct response view with width at least 1"
            )),
        }
    }

    pub fn scroll(mut self, scroll: u16) -> Self {
        self.scroll = scroll;
        self
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn render(&self, area: Rect, frame: &mut Frame) {
        Paragraph::new(self.lines.as_slice())
            .scroll((self.scroll, 0))
            .render(area, frame.buffer_mut());
    }
}
