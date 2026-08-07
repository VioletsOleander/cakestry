use crossterm::event::{MouseEvent, MouseEventKind};

use crate::session::Session;

impl Session {
    pub(super) fn handle_mouse(&mut self, mouse: MouseEvent) {
        match mouse.kind {
            MouseEventKind::ScrollDown => {
                self.scroll = self.scroll.saturating_add(1);
            }
            MouseEventKind::ScrollUp => {
                self.scroll = self.scroll.saturating_sub(1);
            }
            _ => (),
        }
    }
}
