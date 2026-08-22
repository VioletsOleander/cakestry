//! Widget for rendering user input area.
//!
//! Specifically, this module provides [`TextArea`].

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Widget as RatatuiWidget};
use unicode_width::UnicodeWidthStr;

use super::Widget;
use super::symbol::{Angle, Dot};
use super::textwrap;

/// A widget to display wrapped user input text.
pub struct TextArea<'a> {
    /// Flatten collection of `wrapped_lines`.
    lines: Vec<&'a str>,
    /// Lines wrapped by the specified width.
    wrapped_lines: Vec<Vec<&'a str>>,
    /// Prefix width = span width + space width = 2.
    prefix_width: usize,
    /// Scroll offset in y coordinate.
    offset: u16,
    /// Style of the prompt symbol '>' and '.'.
    prompt_style: Style,
}

impl<'a> TextArea<'a> {
    /// Create a `TextArea` from given `lines` and `width`.
    ///
    /// `width` is used to appropriately wrap `lines`.
    pub fn new(lines: &'a [String], width: usize) -> Self {
        let prefix_width = 2;

        let text_width = width
            .checked_sub(prefix_width)
            .expect("Given width for TextArea should be larger than 2.");

        let wrapped_lines: Vec<_> = lines
            .iter()
            .map(|line| textwrap::wrap_line(line, text_width))
            .collect();

        let lines: Vec<_> = wrapped_lines.iter().flatten().copied().collect();

        TextArea {
            lines,
            wrapped_lines,
            prefix_width,
            offset: 0,
            prompt_style: Style::default().fg(Color::Green),
        }
    }

    /// Return the cursor position in the wrapped textarea.
    ///
    /// The returned position is computed from the cursor position in the original lines.
    pub fn locate_cursor(&mut self, position: (usize, usize), width: usize) -> (u16, u16) {
        let (line_idx, byte_idx) = position;
        let wrap_results = &self.wrapped_lines;

        let num_seen_lines: usize = wrap_results.iter().take(line_idx).map(|v| v.len()).sum();
        let mut num_seen_bytes = 0;

        for (i, line) in wrap_results[line_idx].iter().enumerate() {
            // Desired byte's start index is not in current wrapped line.
            if byte_idx > num_seen_bytes + line.len() {
                num_seen_bytes += line.len();
                continue;
            }

            // Desired byte's start index is in current wrapped line.
            let prefix_width = self.prefix_width + line[..(byte_idx - num_seen_bytes)].width();

            // If desired byte's start index is in current wrapped line's end, wrap to next line's
            // first text starting position.
            if prefix_width == width {
                // To ensure the next line has a prompt, an additional empty line is required.
                self.lines.push("");
                return ((prefix_width as u16), (num_seen_lines + i + 1) as u16);
            }

            return ((prefix_width) as u16, (num_seen_lines + i) as u16);
        }

        unreachable!()
    }
}

impl<'a> Widget for TextArea<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [spans_area, paragraph_area] =
            Layout::horizontal([Constraint::Length(1), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);

        for i in 0..spans_area.height {
            let span_area = Rect::new(spans_area.x, spans_area.y + i, spans_area.width, 1);

            // If there is scroll, the first line prompt sign should be skipped.
            if i == 0 && self.offset == 0 {
                Angle::new(self.prompt_style).render(span_area, buf);
            } else {
                Dot.render(span_area, buf);
            }
        }

        Paragraph::new(self.lines.as_slice())
            .scroll((self.offset, 0))
            .render(paragraph_area, buf);
    }

    fn height(&self) -> usize {
        self.lines.len()
    }

    fn scroll(&mut self, offset: u16) {
        self.offset = offset;
    }
}
