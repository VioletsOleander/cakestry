use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Fill, Widget};

use super::Scroll;

pub struct Separator {
    symbol: &'static str,
    scroll: u16,
}

impl Separator {
    pub fn new(symbol: &'static str) -> Self {
        Self { symbol, scroll: 0 }
    }
}

impl Widget for Separator {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.scroll == 0 {
            Fill::new(self.symbol).render(area, buf);
        }
    }
}

impl Scroll for Separator {
    fn scroll(self, scroll: u16) -> Self {
        Self {
            symbol: self.symbol,
            scroll,
        }
    }
}
