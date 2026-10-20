use anyhow::{Result, anyhow};
use async_openai::types::responses::Role;

pub enum MessageSource {
    /// Role under OpenAI message semantics.
    Role(Role),
    /// Tool with its unique call id.
    Tool { call_id: String },
}

pub struct Message {
    pub source: MessageSource,
    pub content: String,
}

impl Message {
    pub fn new(source: MessageSource, content: String) -> Self {
        Self { source, content }
    }

    pub fn push_str(&mut self, content: &str) {
        self.content.push_str(content);
    }
}
