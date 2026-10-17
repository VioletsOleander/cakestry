use anyhow::{Result, anyhow};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Fill, Widget as RatatuiWidget};

use super::Widget;

pub struct SpaceLine {
    scroll: u16,
}

impl SpaceLine {
    pub fn build(width: usize) -> Result<Self> {
        if width > 0 {
            Ok(Self { scroll: 0 })
        } else {
            Err(anyhow!(
                "expected to construct spaceline with width at least 1"
            ))
        }
    }
}

impl Widget for SpaceLine {
    fn scroll(self, scroll: u16) -> Self {
        Self { scroll }
    }

    fn height(&self) -> usize {
        1
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if self.scroll == 0 {
            Fill::new(" ").render(area, buf);
        }
    }
}
