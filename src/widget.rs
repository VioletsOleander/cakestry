pub mod angle;
pub mod dot;
pub mod request;
pub mod response;
pub mod textarea;

pub use angle::Angle;
pub use dot::Dot;
pub use request::Request;
pub use response::Response;
pub use textarea::TextArea;

pub trait HeightMeasurable {
    /// Return the height of this widget.
    fn height(&self, width: u16) -> usize;
}

pub trait Scrollable {
    /// Set the scroll offset of this widget.
    ///
    /// This method follows builer style, which consumes the old instance and returns a new instance.
    fn scroll(self, offset: u16) -> Self;
}
