use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

pub mod query;
pub mod reply;
pub mod separator;

pub use query::Query;
pub use reply::Reply;
pub use separator::Separator;

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

pub trait ReservedWidth {
    /// Return the reserved width for non-text area in itself.
    fn reserved_width() -> usize;
}
