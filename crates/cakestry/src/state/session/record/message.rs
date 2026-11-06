use async_openai::types::responses::Role;

use super::Entry;

pub trait LastMessage {
    /// Return the last message with `role`, panic if not found.
    fn last_message(&mut self, role: Role) -> &mut Message;
}

pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn new(role: Role, content: String) -> Self {
        Self { role, content }
    }

    pub fn push_content(&mut self, delta: &str) {
        self.content.push_str(delta);
    }
}
