use std::cmp;

use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use unicode_width::UnicodeWidthStr;

use super::Session;

mod textwrap;
mod widget;

use textwrap::wrap_line;
use widget::{Query, Reply, Separator, TextArea, Widget};

impl Session {
    /// Render the session onto the given `frame`.
    ///
    /// A [`Session`] will render all stored exchanges and a textarea on a virtual document which
    /// has larger length than the viewport of `frame`. The actually rendered area in the y
    /// coordinate of the document is `[self.view_start, self.view_end)`, where `self.view_start` is
    /// affected by mouse scroll.
    pub fn render(&mut self, frame: &mut Frame) {
        self.offset = 0;
        self.view_end = self.view_start + frame.area().height as usize;

        self.render_exchanges(frame);
        self.render_textarea(frame);
    }

    /// Render exchanges, each in the form of (query, separator, reply, separator).
    fn render_exchanges(&mut self, frame: &mut Frame) {
        for exchange in &self.exchanges {
            self.offset += self.render_widget(
                Query::new(exchange.query(), frame.area().width as usize),
                frame.area(),
                frame.buffer_mut(),
            );

            self.offset +=
                self.render_widget(Separator::default(), frame.area(), frame.buffer_mut());

            self.offset += self.render_widget(
                Reply::new(exchange.reply(), frame.area().width as usize),
                frame.area(),
                frame.buffer_mut(),
            );

            self.offset +=
                self.render_widget(Separator::default(), frame.area(), frame.buffer_mut());
        }
    }

    /// Render the user input area and the screen cursor.
    ///
    /// Render screen cursor requires access to the full `frame`, and compute screen cursor position
    /// requires wrapping text lines. Because only [`Session`] has access to the full `frame`,
    /// [`Session`] has to take the responsibiliy for wrapping text lines for [`TextArea`], unlike
    /// [`Query`] and [`Reply`].
    fn render_textarea(&mut self, frame: &mut Frame) {
        // Wrap lines.
        let text_width = (frame.area().width as usize)
            .checked_sub(TextArea::prefix_width())
            .unwrap_or_else(|| {
                panic!(
                    "Given width for TextArea should be larger than {}",
                    TextArea::prefix_width()
                )
            });
        let mut wrap_results: Vec<_> = (self.user_input.lines())
            .iter()
            .map(|line| wrap_line(line, text_width))
            .collect();

        // Render cursor.
        let (line_idx, byte_idx) = self.cursor.position();

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
            if prefix_width as u16 == frame.area().width {
                frame.set_cursor_position((
                    (TextArea::prefix_width() as u16),
                    (self.offset + num_seen_lines + 1) as u16,
                ));

                // To ensure the next line has a prompt, an additionall empty line is required.
                wrap_results.push(vec![""]);
                break;
            }

            frame.set_cursor_position((
                (prefix_width) as u16,
                (self.offset + num_seen_lines) as u16,
            ));
        }

        // Render lines.
        self.offset += self.render_widget(
            TextArea::new(wrap_results.into_iter().flatten().collect()),
            frame.area(),
            frame.buffer_mut(),
        );
    }

    /// Render `widget` on the visible view in `session_buf`, and return the height of `widget`.
    ///
    /// Because the borrow checker does not allow `render_widget` mutabaly inside the
    /// `&self.exchanges` loop, we have to left the responsibiliy for increasing `self.offset` for
    /// that loop instead of doing it inside this method.
    fn render_widget(
        &self,
        mut widget: impl Widget,
        session_area: Rect,
        session_buf: &mut Buffer,
    ) -> usize {
        let widget_height = widget.height();
        let widget_start = self.offset;
        let widget_end = widget_start + widget_height;

        // Widget lies above the view
        if widget_end <= self.view_start {
            return widget_height;
        }

        // Widget overlaps with view
        let visible_start = cmp::max(self.view_start, widget_start);
        let visible_end = cmp::min(self.view_end, widget_end);

        let offset = visible_start - self.view_start;
        let visible_height = visible_end - visible_start;

        let skip = (visible_start - widget_start) as u16;
        let area = Rect::new(
            session_area.x,
            session_area.y + offset as u16,
            session_area.width,
            visible_height as u16,
        );

        widget.scroll(skip);
        widget.render(area, session_buf);
        widget_height
    }
}
