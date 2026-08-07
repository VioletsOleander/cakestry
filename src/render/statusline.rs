use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Widget;

use super::SessionRenderer;
use crate::session::Session;

impl SessionRenderer {
    pub(super) fn render_statusline(&self, session: &Session, area: Rect, buf: &mut Buffer) {
        // TODO: color theme, true color backgrouned status bar
        let model = session.client().model();
        let line = Line::from(model).right_aligned();

        line.render(area, buf);
    }
}
