use crossterm::event::Event;

use super::Session;

mod key;
mod mouse;

impl Session {
    /// Handle event and modify internal state correspondingly.
    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) => {
                self.handle_key(key);
            }
            Event::Mouse(mouse) => {
                self.handle_mouse(mouse);
            }
            _ => (),
        }
    }
}
