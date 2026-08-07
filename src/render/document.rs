use std::cmp;

use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use unicode_width::UnicodeWidthStr;

use super::SessionRenderer;
use crate::session::Session;
use crate::session::state::{Cursor, Exchange, UserInput};

mod exchange;
mod symbol;
mod textarea;
mod textwrap;

use exchange::{Query, Reply};
use symbol::Separator;
use textarea::TextArea;

impl SessionRenderer {
    /// Render the exchanges and textarea of `session`.
    pub(super) fn render_document(&mut self, session: &Session, area: Rect, frame: &mut Frame) {
        self.document_offset = 0;
        self.document_view = (session.scroll(), session.scroll() + area.height as usize);

        self.render_exchanges(session.exchanges(), area, frame.buffer_mut());
        self.render_textarea(session.user_input(), session.cursor(), area, frame);
    }

    /// Render exchanges on the document, each in the form of (query, separator, reply, separator).
    fn render_exchanges(&mut self, exchanges: &[Exchange], area: Rect, buf: &mut Buffer) {
        for exchange in exchanges {
            self.document_offset +=
                self.render_widget(Query::new(exchange.query(), area.width as usize), area, buf);
            self.document_offset += self.render_widget(Separator::default(), area, buf);

            self.document_offset +=
                self.render_widget(Reply::new(exchange.reply(), area.width as usize), area, buf);
            self.document_offset += self.render_widget(Separator::default(), area, buf);
        }
    }

    /// Render the user input area and the screen cursor.
    fn render_textarea(
        &mut self,
        user_input: &UserInput,
        cursor: &Cursor,
        area: Rect,
        frame: &mut Frame,
    ) {
        // Wrap lines.
        let text_width = (area.width as usize)
            .checked_sub(TextArea::prefix_width())
            .unwrap_or_else(|| {
                panic!(
                    "Given width for TextArea should be larger than {}",
                    TextArea::prefix_width()
                )
            });
        let mut wrap_results: Vec<_> = (user_input.lines())
            .iter()
            .map(|line| textwrap::wrap_line(line, text_width))
            .collect();

        // Render cursor.
        let (line_idx, byte_idx) = cursor.position();

        let mut num_seen_lines: usize = wrap_results.iter().take(line_idx).map(|v| v.len()).sum();
        let mut num_seen_bytes = 0;

        for line in &wrap_results[line_idx] {
            // Desired byte's start index is not in current wrapped line.
            if byte_idx > num_seen_bytes + line.len() {
                num_seen_lines += 1;
                num_seen_bytes += line.len();
                continue;
            }

            // Desired byte's start index is in current wrapped line.
            let prefix_width =
                TextArea::prefix_width() + line[..(byte_idx - num_seen_bytes)].width();

            // If desired byte's start index is in current wrapped line's end, wrap to next line's
            // first text starting position.
            if prefix_width as u16 == area.width {
                frame.set_cursor_position((
                    (TextArea::prefix_width() as u16),
                    (self.document_offset + num_seen_lines + 1) as u16,
                ));

                // To ensure the next line has a prompt, an additionall empty line is required.
                wrap_results.push(vec![""]);
                break;
            }

            frame.set_cursor_position((
                (prefix_width) as u16,
                (self.document_offset + num_seen_lines) as u16,
            ));
        }

        // Render lines.
        self.document_offset += self.render_widget(
            TextArea::new(wrap_results.into_iter().flatten().collect()),
            area,
            frame.buffer_mut(),
        );
    }

    /// Render `widget` on the visible view in `document_area`, and return the height of `widget`.
    ///
    /// Because the borrow checker does not allow `render_widget` mutabaly inside the
    /// `&self.exchanges` loop, we have to left the responsibiliy for increasing `self.offset` for
    /// that loop instead of doing it inside this method.
    fn render_widget(
        &self,
        mut widget: impl Widget,
        document_area: Rect,
        document_buf: &mut Buffer,
    ) -> usize {
        let (view_start, view_end) = self.document_view;

        let widget_height = widget.height();
        let widget_start = self.document_offset;
        let widget_end = widget_start + widget_height;

        // Widget lies above the view
        if widget_end <= view_start {
            return widget_height;
        }

        // Widget overlaps with view
        let visible_start = cmp::max(view_start, widget_start);
        let visible_end = cmp::min(view_end, widget_end);

        let offset = visible_start - view_start;
        let visible_height = visible_end - visible_start;

        let skip = (visible_start - widget_start) as u16;
        let area = Rect::new(
            document_area.x,
            document_area.y + offset as u16,
            document_area.width,
            visible_height as u16,
        );

        widget.scroll(skip);
        widget.render(area, document_buf);
        widget_height
    }
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
