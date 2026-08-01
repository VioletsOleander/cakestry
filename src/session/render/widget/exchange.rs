//! Widgets for rendering exchange.
//!
//! Specifically, this module provides [`Query`] and [`Reply`].

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Widget as RatatuiWidget};

use super::Widget;
use super::symbol::{Angle, Dot};
use crate::session::render::textwrap::wrap_line;

/// A widget to display wrapped user input text.
#[derive(Debug)]
pub struct Query<'a> {
    /// Lines of text wrapped by the specified width.
    lines: Vec<&'a str>,
    /// Scroll offset in y coordinate.
    offset: u16,
    /// Style of the prompt symbol '>' and '.'.
    prompt_style: Style,
}

impl<'a> Query<'a> {
    /// Create a `Query` from given `lines` and `width`.
    ///
    /// `width` is used to appropriately wrap `lines`.
    ///
    /// Since widget is used as a render command, pass all the command needs (`lines`, `width`) to
    /// make a render as arguments here is reasonable.
    pub fn new(lines: &'a [String], width: usize) -> Self {
        // prefix width = span width + space width = 2
        let text_width = width
            .checked_sub(2)
            .expect("Given width for Query should be larger than 2");

        let wrapped_lines = lines
            .iter()
            .flat_map(|line| wrap_line(line, text_width))
            .collect();

        Query {
            lines: wrapped_lines,
            offset: 0,
            prompt_style: Style::default().fg(Color::Blue),
        }
    }
}

impl<'a> Widget for Query<'a> {
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

/// A widget to display assistant reply.
#[derive(Debug)]
pub struct Reply<'a> {
    /// Lines of text wrapped by the specified width.
    lines: Vec<&'a str>,
    /// Scroll offset in y coordinate.
    offset: u16,
}

impl<'a> Reply<'a> {
    /// Create a `Reply` from given `lines` and `width`.
    ///
    /// `width` is used to appropriately wrap `lines`.
    pub fn new(lines: &'a [String], width: usize) -> Self {
        let wrapped_lines = lines
            .iter()
            .flat_map(|line| wrap_line(line, width))
            .collect();

        Reply {
            lines: wrapped_lines,
            offset: 0,
        }
    }
}

impl<'a> Widget for Reply<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(self.lines.as_slice())
            .scroll((self.offset, 0))
            .render(area, buf);
    }

    fn height(&self) -> usize {
        self.lines.len()
    }

    fn scroll(&mut self, offset: u16) {
        self.offset = offset;
    }
}
