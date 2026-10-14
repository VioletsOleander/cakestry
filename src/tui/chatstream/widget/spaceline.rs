use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Fill, Widget as RatatuiWidget};

use super::Widget;

#[derive(Default)]
pub struct SpaceLine {
    scroll: u16,
}

impl Widget for SpaceLine {
    fn scroll(self, scroll: u16) -> Self {
        Self { scroll }
    }

    fn height(&self) -> usize {
        1
    }

    fn render(&self, area: Rect, frame: &mut Frame) {
        if self.scroll == 0 {
            Fill::new(" ").render(area, frame.buffer_mut());
        }
    }
}
