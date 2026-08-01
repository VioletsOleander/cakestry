use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

mod exchange;
mod symbol;
mod textarea;

pub use exchange::{Query, Reply};
pub use symbol::Separator;

/// A type that can be drawn on the document and the viewport in the session.
pub trait Widget {
    /// Render itself to the specified area and buffer.
    fn render(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized;

    /// Return the height of itself.
    fn height(&self) -> usize;

    /// Set the scroll offset of itself.
    fn scroll(&mut self, offset: u16);
}
