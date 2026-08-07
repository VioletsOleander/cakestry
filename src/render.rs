use ratatui::Frame;

use super::session::Session;

mod document;
mod statusline;

#[derive(Default)]
pub struct SessionRenderer {
    offset: usize,
    view: (usize, usize),
}

impl SessionRenderer {
    /// Render `session` on the given `frame`.
    ///
    /// Exchanges and textarea of `session` will be renderd on a virtual document, which has larger
    /// length than the viewport of `frame`. The actually rendered area starts from `session`'s
    /// scroll offest.
    pub fn render(&mut self, session: &Session, frame: &mut Frame) {
        self.offset = 0;
        self.view = (
            session.scroll(),
            session.scroll() + frame.area().height as usize,
        );

        self.render_document(session, frame);
    }
}
