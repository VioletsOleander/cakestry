use std::cmp;

use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use super::Session;

mod widget;
mod wrap;

use widget::{Query, Reply, ReservedWidth, Separator, Widget};
use wrap::wrap_line;

impl Session {
    /// Render the session onto the given `frame`.
    ///
    /// A `Session` will virtually render all exchanges on a virtual document with larger length
    /// than the viewport of the `frame`. The actually rendered area in the y coordinate of the
    /// document is `[self.view_start, self.view_end)`, where `self.view_start` is affected by mouse
    /// scroll.
    pub fn render(&mut self, frame: &mut Frame) {
        self.offset = 0;
        self.view_end = self.view_start + frame.area().height as usize;

        self.render_exchanges(frame);
        self.render_input_area(frame);
    }

    fn render_exchanges(&mut self, frame: &mut Frame) {
        // Render (query, seperator, reply, seperator).
        for exchange in &self.exchanges {
            // TODO: make this long part looks better, the performance is ok now.
            if self.offset >= self.view_end {
                return;
            } else {
                let text_width = (frame.area().width as usize)
                    .checked_sub(Query::reserved_width())
                    .expect("Reserved width should be less than area width");

                let query_lines = exchange.query_lines();
                let wrapped_lines = query_lines
                    .iter()
                    .flat_map(|line| wrap_line(line, text_width))
                    .collect();

                let mut query = Query::new(wrapped_lines);
                query.set_prompt_style(Style::default().fg(Color::Blue));

                self.offset += self.render_widget(query, frame.area(), frame.buffer_mut());
            }

            if self.offset >= self.view_end {
                return;
            } else {
                self.offset +=
                    self.render_widget(Separator::default(), frame.area(), frame.buffer_mut());
            }

            if self.offset >= self.view_end {
                return;
            } else {
                let text_width = (frame.area().width as usize)
                    .checked_sub(Reply::reserved_width())
                    .expect("Reserved width should be less than area width");

                let reply_lines = exchange.reply_lines();
                let wrapped_lines = reply_lines
                    .iter()
                    .flat_map(|line| wrap_line(line, text_width))
                    .collect();

                let reply = Reply::new(wrapped_lines);

                self.offset += self.render_widget(reply, frame.area(), frame.buffer_mut());
            }

            if self.offset >= self.view_end {
                return;
            } else {
                self.offset +=
                    self.render_widget(Separator::default(), frame.area(), frame.buffer_mut());
            }
        }
    }

    fn render_input_area(&mut self, frame: &mut Frame) {
        if self.offset >= self.view_end {
            return;
        }

        let text_width = (frame.area().width as usize)
            .checked_sub(Query::reserved_width())
            .expect("Reserved width should be less than area width");

        let input_lines = self.user_input.lines();
        let wrap_results: Vec<_> = input_lines
            .iter()
            .map(|line| wrap_line(line, text_width))
            .collect();

        // let (line_idx, byte_idx) = self.cursor.position();

        // let mut num_seen_lines = 0;
        // for i in 0..line_idx {
        //     num_seen_lines += wrap_results[i].len();
        // }
        //
        // let mut num_seen_bytes = 0;
        // for wrapped_line in wrap_results[line_idx] {
        //     num_seen_bytes += wrapped_line.len();
        //
        //     // In this line
        //     if byte_idx < num_seen_bytes {}
        //
        //     // In next line
        //     if byte_idx >= num_seen_bytes {
        //         num_seen_lines += 1;
        //         num_seen_bytes += 1; // Additional newline char
        //     }
        // }

        // let wrapped_lines = wrap_results.iter().flatten().collect();
        let mut widget = Query::new(wrapped_lines);
        widget.set_prompt_style(Style::default().fg(Color::Green));

        let height = self.render_widget(widget, frame.area(), frame.buffer_mut());
        self.offset += height;
    }

    /// Render `widget` onto the visible view in `session_buf`, and return the height of `widget`.
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
