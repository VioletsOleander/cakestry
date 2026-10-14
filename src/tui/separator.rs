use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Fill, Widget};

pub struct Separator;

impl Separator {
    pub fn render(area: Rect, frame: &mut Frame) {
        Fill::new("_").render(area, frame.buffer_mut());
    }
}
