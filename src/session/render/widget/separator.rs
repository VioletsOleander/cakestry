use super::Widget;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Fill, Widget as RatatuiWidget};

/// A widget to display an empty line as separator.
#[derive(Default)]
pub struct Separator {}

impl Widget for Separator {
    fn render(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Fill::new(" ").render(area, buf);
    }

    fn height(&self) -> usize {
        1
    }

    fn scroll(&mut self, _offset: u16) {}
}
