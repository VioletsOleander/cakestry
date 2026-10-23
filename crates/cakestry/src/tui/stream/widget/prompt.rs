use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::Widget;

use super::Scroll;

pub struct Prompt {
    style: Style,
    scroll: u16,
}

impl Prompt {
    pub fn new(style: Style) -> Self {
        Self { style, scroll: 0 }
    }
}

impl Widget for Prompt {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for i in 0..area.height {
            let span_area = Rect {
                y: area.y + i,
                height: 1,
                ..area
            };

            if i == 0 && self.scroll == 0 {
                Span::styled(">", self.style).render(span_area, buf);
            } else {
                Span::styled(".", self.style).render(span_area, buf);
            }
        }
    }
}

impl Scroll for Prompt {
    fn scroll(self, scroll: u16) -> Self {
        Self {
            style: self.style,
            scroll,
        }
    }
}
