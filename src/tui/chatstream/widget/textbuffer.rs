use anyhow::{Result, anyhow};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Paragraph, Widget as RatatuiWidget};
use unicode_width::UnicodeWidthStr;

use super::Widget;
use crate::state::buffer::Cursor;
use crate::tui::chatstream::textwrap::wrap_line;

pub struct TextBuffer<'a> {
    lines: Vec<&'a str>,
    cursor: Cursor,
    scroll: u16,
}

impl<'a> TextBuffer<'a> {
    pub fn build(lines: &'a [String], cursor: Cursor, width: usize) -> Result<Self> {
        // 1 marker width + 1 space = 2
        let prefix_width = 2;

        match width.checked_sub(prefix_width) {
            Some(text_width) => {
                let line_wraps: Vec<_> = lines
                    .iter()
                    .map(|line| wrap_line(line, text_width))
                    .collect();

                // `try_fold` must return Err, otherwise it means the buffer cursor char is not
                // located in the buffer cursor line, which implies code logic error.
                let (cursor_line, cursor_char) = line_wraps[cursor.line]
                    .iter()
                    .try_fold(
                        (
                            // Number of visual lines before the buffer cursor line.
                            line_wraps
                                .iter()
                                .take(cursor.line)
                                .map(|wrap| wrap.len())
                                .sum(),
                            // Number of actual bytes before the buffer cursor char.
                            0,
                        ),
                        |(cursor_line, num_bytes), &wrap| {
                            let wrap_end = num_bytes + wrap.len();

                            if cursor.char > wrap_end {
                                // If the actual cursor char is beyond the current wrap, goto next wrap.
                                Ok((cursor_line + 1, wrap_end))
                            } else {
                                // If the actual cursor char is in current wrap, compute visual cursor char
                                // in this wrap (the visual length of the preifx string) and return it.
                                let cursor_char = wrap[..(cursor.char - num_bytes)].width();

                                Err((cursor_line, cursor_char))
                            }
                        },
                    )
                    .unwrap_err();

                Ok(Self {
                    lines: line_wraps.into_iter().flatten().collect(),
                    cursor: Cursor::new(cursor_line, cursor_char),
                    scroll: 0,
                })
            }
            None => Err(anyhow!("failed to construct buffer view with given width")),
        }
    }
}

impl<'a> Widget for TextBuffer<'a> {
    fn scroll(self, scroll: u16) -> Self {
        Self {
            lines: self.lines,
            cursor: self.cursor,
            scroll,
        }
    }

    fn height(&self) -> usize {
        self.lines.len()
    }

    fn render(&self, area: Rect, frame: &mut Frame) {
        let [spans_area, paragraph_area] =
            Layout::horizontal([Constraint::Length(1), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);

        for i in 0..spans_area.height {
            let span_area = Rect::new(spans_area.x, spans_area.y + i, spans_area.width, 1);

            if i == 0 && self.scroll == 0 {
                Span::styled(">", Style::default().fg(Color::Green))
                    .render(span_area, frame.buffer_mut());
            } else {
                Span::styled(".", Style::default().fg(Color::Green))
                    .render(span_area, frame.buffer_mut());
            }
        }

        Paragraph::new(self.lines.as_slice())
            .scroll((self.scroll, 0))
            .render(paragraph_area, frame.buffer_mut());

        if let Some(y_offset) = (self.cursor.line as u16).checked_sub(self.scroll)
            && y_offset < area.height
            && let x_offset = self.cursor.char as u16 + 2 // 2 for prefix width
            && x_offset < area.width
        {
            let cursor_position = (area.x + x_offset, area.y + y_offset);
            frame.set_cursor_position(cursor_position);
        }
    }
}
