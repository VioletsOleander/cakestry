use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::Widget as RatatuiWidget;

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
