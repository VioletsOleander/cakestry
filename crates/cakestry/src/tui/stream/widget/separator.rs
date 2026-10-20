use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Fill, Widget as RatatuiWidget};

use super::Widget;

pub struct Separator {
    scroll: u16,
}

impl Separator {
    pub fn new() -> Self {
        Self { scroll: 0 }
    }
}

impl Widget for Separator {
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
