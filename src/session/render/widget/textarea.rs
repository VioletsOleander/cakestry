//! Widget for rendering user input area.
//!
//! Specifically, this module provides [`TextArea`].

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Widget as RatatuiWidget};

use super::Widget;
use super::symbol::{Angle, Dot};

/// A widget to display wrapped user input text.
///
/// Different from [`super::Query`], `TextArea` delegates the responsiblity of line wrapping to the
/// session, and itself is only responsible for rendering those wrapped lines.
pub struct TextArea<'a> {
    /// Lines of text wrapped by the specified width
    wrapped_lines: Vec<&'a str>,
    /// Scroll offset in y coordinate
    offset: u16,
    /// Style of the prompt symbol '>' and '.'
    prompt_style: Style,
}

impl<'a> TextArea<'a> {
    /// Create a `TextArea` from given `wrapped_lines`.
    pub fn new(wrapped_lines: Vec<&'a str>) -> Self {
        TextArea {
            wrapped_lines,
            offset: 0,
            prompt_style: Style::default().fg(Color::Green),
        }
    }

    /// Return the width of `TextArea`'s prefix non-text area.
    pub fn prefix_width() -> usize {
        // 1 span width + 1 space width
        2
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

        Paragraph::new(self.wrapped_lines.as_slice())
            .scroll((self.offset, 0))
            .render(paragraph_area, buf);
    }

    fn height(&self) -> usize {
        self.wrapped_lines.len()
    }

    fn scroll(&mut self, offset: u16) {
        self.offset = offset;
    }
}
