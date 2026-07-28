use super::Session;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use std::borrow::Cow;
use std::cmp;

mod widget;

use widget::{Query, Reply, ReservedWidth, Separator, Widget};

impl Session {
    /// Display the session on the given `frame`.
    ///
    /// A `Session` will virtually render all exchanges on a virtual document with larger length than the viewport of
    /// the `frame`. The actually rendered area in the y coordinate of the document is `[self.view_start,
    /// self.view_end)`, where `self.view_start` is affected by mouse scroll.
    pub fn display(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let buf = frame.buffer_mut();

        self.offset = 0;
        self.view_end = self.view_start + area.height as usize;

        // Render (query, seperator, reply, seperator).
        for exchange in &self.exchanges {
            if self.offset >= self.view_end {
                return;
            } else {
                let wrapped_lines = self.wrap_lines(
                    exchange.query_lines(),
                    Query::reserved_width(),
                    area.width as usize,
                );
                let mut widget = Query::new(wrapped_lines);
                widget.set_prompt_style(Style::default().fg(Color::Blue));

                let height = self.render_widget(widget, area, buf);
                self.offset += height;
            }

            if self.offset >= self.view_end {
                return;
            } else {
                let widget = Separator::default();

                let height = self.render_widget(widget, area, buf);
                self.offset += height;
            }

            if self.offset >= self.view_end {
                return;
            } else {
                let wrapped_lines = self.wrap_lines(
                    exchange.reply_lines(),
                    Reply::reserved_width(),
                    area.width as usize,
                );
                let widget = Reply::new(wrapped_lines);

                let height = self.render_widget(widget, area, buf);
                self.offset += height;
            }

            if self.offset >= self.view_end {
                return;
            } else {
                let widget = Separator::default();

                let height = self.render_widget(widget, area, buf);
                self.offset += height;
            }
        }

        // Render user input area.
        if self.offset >= self.view_end {
            return;
        } else {
            let wrapped_lines = self.wrap_lines(
                self.user_input.lines(),
                Query::reserved_width(),
                area.width as usize,
            );
            let mut widget = Query::new(wrapped_lines);
            widget.set_prompt_style(Style::default().fg(Color::Green));

            let height = self.render_widget(widget, area, buf);
            self.offset += height;
        }
    }

    fn wrap_lines<'a>(
        &'a self,
        lines: &'a [String],
        reserved_with: usize,
        area_width: usize,
    ) -> Vec<Cow<'a, str>> {
        let text_width = area_width
            .checked_sub(reserved_with)
            .expect("Reserved width should be less than area width");

        lines
            .iter()
            .flat_map(|line| textwrap::wrap(line, text_width))
            .collect()
    }

    /// Render `widget` into the visible view in `session_buf`, and return the height of `widget`.
    ///
    /// Because the borrow checker does not allow `render_widget` mutabaly inside the `&self.exchanges` loop, we have to
    /// left the responsibiliy for increasing `self.offset` for that loop instead of doing it inside this method.
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
