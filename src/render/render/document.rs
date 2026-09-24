use std::cmp;

use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use crate::session::Session;

mod exchange;
mod symbol;
mod textarea;
mod textwrap;

use exchange::{Query, Reply};
use symbol::Separator;
use textarea::TextArea;

pub struct Document<'a> {
    session: &'a Session,
}

/// A type that can be drawn on the document in a session.
trait Widget {
    /// Render itself to the specified area and buffer.
    fn render(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized;

    /// Return the height of itself.
    fn height(&self) -> usize;

    /// Set the scroll offset of itself.
    fn scroll(&mut self, offset: u16);
}

struct Layout {
    scroll: u16,
    area: Rect,
}

impl<'a> Document<'a> {
    pub fn new(session: &'a Session) -> Self {
        Self { session }
    }

    pub fn render(&mut self, area: Rect, frame: &mut Frame) {
        // Absolute offset.
        let mut offset = 0;
        // Absolute range of the visible view.
        let view = (
            self.session.scroll(),
            self.session.scroll() + area.height as usize,
        );

        let buf = frame.buffer_mut();
        let width = area.width as usize;

        // Each exchange in the form of (query, separator, reply, separator).
        for exchange in self.session.exchanges() {
            {
                let mut widget = Query::new(exchange.query(), width);
                if let Some(layout) = self.compute_layout(widget.height(), offset, view, area) {
                    widget.scroll(layout.scroll);
                    widget.render(layout.area, buf);
                }

                offset += widget.height();
            }

            {
                let mut widget = Separator::default();
                if let Some(layout) = self.compute_layout(widget.height(), offset, view, area) {
                    widget.scroll(layout.scroll);
                    widget.render(layout.area, buf);
                }

                offset += widget.height();
            }

            {
                let mut widget = Reply::new(exchange.reply(), width);
                if let Some(layout) = self.compute_layout(widget.height(), offset, view, area) {
                    widget.scroll(layout.scroll);
                    widget.render(layout.area, buf);
                }

                offset += widget.height();
            }

            {
                let mut widget = Separator::default();
                if let Some(layout) = self.compute_layout(widget.height(), offset, view, area) {
                    widget.scroll(layout.scroll);
                    widget.render(layout.area, buf);
                }

                offset += widget.height();
            }
        }

        let mut textarea = TextArea::new(self.session.user_input().lines(), width);
        if let Some(layout) = self.compute_layout(textarea.height(), offset, view, area) {
            textarea.scroll(layout.scroll);
            textarea.render(layout.area, buf);

            let mut cursor_position =
                textarea.locate_cursor(self.session.cursor().position(), width);
            cursor_position.1 = layout.area.y + cursor_position.1 - layout.scroll;

            if cursor_position.1 <= area.height {
                frame.set_cursor_position(cursor_position);
            }
        }
    }

    /// Return the necessary parameters to render a widget with height `height`.
    fn compute_layout(
        &self,
        widget_height: usize,
        offset: usize,
        view: (usize, usize),
        area: Rect,
    ) -> Option<Layout> {
        let (view_start, view_end) = view;
        let (widget_start, widget_end) = (offset, offset + widget_height);

        // Widget lies above/below the view
        if widget_end <= view_start || widget_start >= view_end {
            return None;
        }

        // Widget overlaps with view
        let visible_start = cmp::max(view_start, widget_start);
        let visible_end = cmp::min(view_end, widget_end);

        let offset = visible_start - view_start;
        let visible_height = visible_end - visible_start;

        let scroll = (visible_start - widget_start) as u16;
        let widget_area = Rect::new(
            area.x,
            area.y + offset as u16,
            area.width,
            visible_height as u16,
        );

        Some(Layout {
            scroll,
            area: widget_area,
        })
    }
}
