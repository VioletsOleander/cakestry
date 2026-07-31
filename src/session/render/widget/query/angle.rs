use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::Widget as RatatuiWidget;

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
