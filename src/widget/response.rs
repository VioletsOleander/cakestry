use crate::widget::{HeightMeasurable, Scrollable};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Widget};

/// A widget to display assistant output text.
pub struct Response<'a> {
    /// Container of assistant output text.
    paragraph: Paragraph<'a>,
}

impl<'a> Response<'a> {
    /// Return a new `Response` containing given `Paragraph`.
    pub fn new(paragraph: Paragraph<'a>) -> Self {
        Response {
            paragraph: paragraph,
        }
    }
}

impl<'a> HeightMeasurable for Response<'a> {
    /// Return the height of this widget.
    fn height(&self, width: u16) -> usize {
        self.paragraph.line_count(width)
    }
}
impl<'a> Scrollable for Response<'a> {
    fn scroll(self, offset: u16) -> Self {
        Response {
            paragraph: self.paragraph.scroll((offset, 0)),
        }
    }
}

impl<'a> Widget for Response<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        (&self.paragraph).render(area, buf);
    }
}
