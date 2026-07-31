use std::cmp;

use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use unicode_width::UnicodeWidthStr;

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
            if self.offset >= self.view_end {
                return;
            } else {
                let text_width = (frame.area().width as usize)
                    .checked_sub(Query::reserved_width())
                    .expect("Reserved width should be less than area width");

                let query_lines = exchange.query_lines();
                let lines = query_lines
                    .iter()
                    .flat_map(|line| wrap_line(line, text_width))
                    .collect();

                let mut query = Query::new(lines);
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
                let lines = reply_lines
                    .iter()
                    .flat_map(|line| wrap_line(line, text_width))
                    .collect();

                let reply = Reply::new(lines);

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

        // Render cursor
        let (line_idx, byte_idx) = self.cursor.position();

        let mut num_seen_lines = 0;
        for lines in wrap_results.iter().take(line_idx) {
            num_seen_lines += lines.len();
        }

        let mut num_seen_bytes = 0;
        for line in &wrap_results[line_idx] {
            if byte_idx > num_seen_bytes + line.len() {
                num_seen_lines += 1;
                num_seen_bytes += line.len();
            } else {
                let visual_byte_idx = byte_idx - num_seen_bytes;
                let prefix_width = line[..visual_byte_idx].width();

                // workaround for showing cursor in next line when current line is full
                // the beam must be showd after the char, so an additional line is necessary
                // needs refactor, since the prefix prompt still not shown, because there is no acutal data
                // maybe need an additional widget for input area, which has its own logic.
                if (prefix_width + Query::reserved_width()) as u16 == frame.area().width {
                    let visual_line_idx = num_seen_lines + 1;
                    let prefix_width = 0;

                    frame.set_cursor_position((
                        (prefix_width + Query::reserved_width()) as u16,
                        (self.offset + visual_line_idx) as u16,
                    ));
                } else {
                    let visual_line_idx = num_seen_lines;

                    frame.set_cursor_position((
                        (prefix_width + Query::reserved_width()) as u16,
                        (self.offset + visual_line_idx) as u16,
                    ));
                }
            }
        }

        // Render lines
        let lines = wrap_results.into_iter().flatten().collect();
        let mut query = Query::new(lines);
        query.set_prompt_style(Style::default().fg(Color::Green));

        self.offset += self.render_widget(query, frame.area(), frame.buffer_mut());
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
