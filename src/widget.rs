pub mod angle;
pub mod dot;
pub mod query;
pub mod reply;

pub use angle::Angle;
pub use dot::Dot;
pub use query::Query;
pub use reply::Reply;

pub trait Renderable {
    /// Render but not consume itself.
    fn render(&self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer)
    where
        Self: Sized;
}

pub trait HeightMeasurable {
    /// Return the height of this widget.
    fn height(&self, width: u16) -> usize;
}

pub trait Scrollable {
    /// Set the scroll offset of this widget.
    fn scroll(&mut self, offset: u16);
}
