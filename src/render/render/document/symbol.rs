//! Widgets for rendering non-text symbols.
//!
//! Specifically, this module provides [`Angle`], [`Dot`], and [`Separator`].

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Fill, Widget as RatatuiWidget};

use super::Widget;

/// A widget to display an empty line as separator.
#[derive(Default)]
pub struct Separator {}

impl Widget for Separator {
    fn render(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Fill::new(" ").render(area, buf);
    }

    fn height(&self) -> usize {
        1
    }

    fn scroll(&mut self, _offset: u16) {}
}

/// A widget to display a `>` symbol.
#[derive(Default)]
pub struct Angle {
    style: Style,
}

impl Angle {
    pub fn new(style: Style) -> Self {
        Angle { style }
    }
}

impl RatatuiWidget for Angle {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let span = Span::styled(">", self.style);

        span.render(area, buf);
    }
}

/// A widget to display a `.` symbol.
#[derive(Default)]
pub struct Dot;

impl RatatuiWidget for Dot {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let style = Style::default().fg(Color::Blue);
        let span = Span::styled(".", style);

        span.render(area, buf);
    }
}
