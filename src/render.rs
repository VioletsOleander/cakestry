use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};

use super::session::Session;

mod document;
mod statusline;

#[derive(Default)]
pub struct SessionRenderer {
    document_offset: usize,
    document_view: (usize, usize),
}

impl SessionRenderer {
    /// Render `session` on the given `frame`.
    ///
    /// Exchanges and textarea of `session` will be renderd on a virtual document, which has larger
    /// length than the viewport of `frame`. The actually rendered area starts from `session`'s
    /// scroll offest.
    pub fn render(&mut self, session: &Session, frame: &mut Frame) {
        let [document_area, statusline_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());

        self.render_document(session, document_area, frame);
        self.render_statusline(session, statusline_area, frame.buffer_mut());
    }
}
