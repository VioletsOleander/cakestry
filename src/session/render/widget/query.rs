use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Paragraph, Widget as RatatuiWidget};

use super::{ReservedWidth, Widget};

mod angle;
mod dot;

use angle::Angle;
use dot::Dot;

/// A widget to display wrapped user input text.
#[derive(Debug)]
pub struct Query<'a> {
    wrapped_lines: Vec<&'a str>,
    /// Scroll offset in y coordinate
    offset: u16,
    /// Style of the prompt symbol '>'
    prompt_style: Style,
}

impl<'a> Query<'a> {
    /// Create a `Query` from given lines.
    pub fn new(wrapped_lines: Vec<&'a str>) -> Self {
        Query {
            wrapped_lines,
            offset: 0,
            prompt_style: Style::default(),
        }
    }

    /// Set the style of the prompt sign.
    pub fn set_prompt_style(&mut self, style: Style) {
        self.prompt_style = style;
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

            if i == 0 && self.offset == 0 {
                let prompt = Angle::new(self.prompt_style);
                prompt.render(span_area, buf);
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

impl<'a> ReservedWidth for Query<'a> {
    fn reserved_width() -> usize {
        // 1 width span + 1 width space
        2
    }
}
