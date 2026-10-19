use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

mod buffer;
mod paragraph;
mod prompt;
mod separator;

pub use buffer::TextBuffer;
pub use paragraph::Paragraph;
pub use prompt::Prompt;
pub use separator::Separator;

pub trait Widget {
    fn render(&self, area: Rect, buf: &mut Buffer);

    fn height(&self) -> usize;

    fn scroll(self, scroll: u16) -> Self;
}
