use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Widget;

use super::TerminalRenderer;
use crate::client::Client;

impl TerminalRenderer {
    pub(super) fn render_statusline(&self, client: &Client, area: Rect, buf: &mut Buffer) {
        let model = client.model();
        let line = Line::from(model).right_aligned();

        line.render(area, buf);
    }
}
