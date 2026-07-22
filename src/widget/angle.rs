use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::Widget;

#[derive(Default)]
pub struct Angle;

impl Widget for Angle {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let style = Style::default().fg(Color::Green);
        let span = Span::styled(">", style);

        span.render(area, buf);
    }
}
