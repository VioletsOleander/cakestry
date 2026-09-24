use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::text::Line;
use ratatui::widgets::Widget;

use crate::service::Service;
use crate::session::Session;

mod document;

use document::Document;

#[derive(Default)]
pub struct TerminalRenderer {}

impl TerminalRenderer {
    /// Render the visible part of the document and a status line on the given `frame`.
    ///
    /// Exchanges and textarea of `session` will be rendered on a virtual document, which has larger
    /// length than the viewport of `frame`. The actually rendered area starts from `session`'s
    /// scroll offset.
    pub fn render(&mut self, session: &Session, service: &Service, frame: &mut Frame) {
        let [document_area, statusline_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());

        let mut document = Document::new(session);
        document.render(document_area, frame);

        let status_line = Line::from(service.model()).right_aligned();
        status_line.render(statusline_area, frame.buffer_mut());
    }
}
