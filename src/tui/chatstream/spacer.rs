use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Fill, Widget};

pub struct Spacer;

impl Spacer {
    pub fn render(area: Rect, frame: &mut Frame) {
        Fill::new(" ").render(area, frame.buffer_mut());
    }
}
