use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Widget;

use super::TerminalRenderer;
use crate::service::Service;

impl TerminalRenderer {
    pub(super) fn render_statusline(&self, service: &Service, area: Rect, buf: &mut Buffer) {
        let model = service.model();
        let line = Line::from(model).right_aligned();

        line.render(area, buf);
    }
}
