use crate::widget::{Angle, Dot, HeightMeasurable, Scrollable};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Paragraph, Widget};

/// A widget to display user input text.
pub struct Request<'a> {
    /// Container of user input text
    paragraph: Paragraph<'a>,
    /// Scroll offset in y coordinate
    offset: u16,
}

impl<'a> Request<'a> {
    /// Return a new `Request` containing given `Paragraph`.
    pub fn new(paragraph: Paragraph<'a>) -> Self {
        Request {
            paragraph: paragraph,
            offset: 0,
        }
    }
}

impl<'a> HeightMeasurable for Request<'a> {
    /// Return the height of this widget.
    fn height(&self, width: u16) -> usize {
        // span width + space width = 2
        if width < 2 {
            return 0;
        }

        self.paragraph.line_count(width - 2)
    }
}

impl<'a> Scrollable for Request<'a> {
    fn scroll(self, offset: u16) -> Self {
        Request {
            paragraph: self.paragraph.scroll((offset, 0)),
            offset: offset,
        }
    }
}

impl<'a> Widget for Request<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [spans_area, paragraph_area] =
            Layout::horizontal([Constraint::Length(1), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);

        for i in 0..spans_area.height {
            if i < self.offset {
                continue;
            }

            let span_area = Rect::new(spans_area.x, spans_area.y + i, spans_area.width, 1);
            if i == 0 {
                Angle.render(span_area, buf);
            } else {
                Dot.render(span_area, buf);
            }
        }

        (&self.paragraph).render(paragraph_area, buf);
    }
}
