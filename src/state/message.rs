use anyhow::{Result, anyhow};
use async_openai::types::responses::Role;

pub enum MessageSource {
    /// Role under OpenAI message semantics.
    Role(Role),
    /// Tool with its unique call id.
    Tool { call_id: String },
}

pub struct MessageStore {
    pub commits: Vec<Message>,
    pub staged: Option<Message>,
}

pub struct Message {
    pub source: MessageSource,
    pub content: String,
}

impl MessageStore {
    pub fn new(commits: Vec<Message>) -> Self {
        Self {
            commits,
            staged: None,
        }
    }

    pub fn commit(&mut self, message: Message) {
        self.commits.push(message);
    }

    pub fn stage(&mut self, message: Message) {
        self.staged = Some(message);
    }

    pub fn stage_delta(&mut self, delta: &str) -> Result<()> {
        match self.staged.as_mut() {
            Some(message) => {
                message.push_str(delta);
                Ok(())
            }
            None => Err(anyhow!("failed to stage delta, there is no staged message")),
        }
    }

    pub fn commit_staged(&mut self) -> Result<()> {
        let staged = self.staged.take();

        match staged {
            Some(message) => {
                self.commits.push(message);
                Ok(())
            }
            None => Err(anyhow!(
                "failed to commit staged message, there is no staged message"
            )),
        }
    }

    pub fn abort_staged(&mut self) {
        self.staged = None;
    }
}

impl Message {
    pub fn new(source: MessageSource, content: String) -> Self {
        Self { source, content }
    }

    pub fn push_str(&mut self, content: &str) {
        self.content.push_str(content);
    }
}
