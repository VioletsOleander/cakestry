use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

mod prompt;
mod response;
mod spaceline;
mod textbuffer;

pub use prompt::Prompt;
pub use response::Response;
pub use spaceline::SpaceLine;
pub use textbuffer::TextBuffer;

pub trait Widget {
    fn render(&self, area: Rect, buf: &mut Buffer);

    fn height(&self) -> usize;

    fn scroll(self, scroll: u16) -> Self;
}
