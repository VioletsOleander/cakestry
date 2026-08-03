use crossterm::event::{MouseEvent, MouseEventKind};

use crate::session::Session;

impl Session {
    pub(super) fn handle_mouse(&mut self, mouse: MouseEvent) {
        match mouse.kind {
            MouseEventKind::ScrollDown => {
                self.view_start = self.view_start.saturating_add(1);
            }
            MouseEventKind::ScrollUp => {
                self.view_start = self.view_start.saturating_sub(1);
            }
            _ => (),
        }
    }
}
