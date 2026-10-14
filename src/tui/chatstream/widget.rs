use ratatui::Frame;
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
    fn render(&self, area: Rect, frame: &mut Frame);

    fn height(&self) -> usize;

    fn scroll(self, scroll: u16) -> Self;
}
