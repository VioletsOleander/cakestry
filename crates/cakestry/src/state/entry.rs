use async_openai::types::responses::{FunctionToolCall, OutputMessage, OutputMessageContent, Role};

pub trait LastMessage {
    /// Return the last message with `role`, panic if not found.
    fn last_message(&mut self, role: Role) -> &mut Message;
}

pub trait LastToolCall {
    /// Return the last tool call, panic if not found.
    fn last_toolcall(&mut self) -> &mut ToolCall;
}

pub enum Entry {
    Message(Message),
    ToolCall(ToolCall),
    ToolCallResult(ToolCallResult),
}

pub struct Message {
    pub role: Role,
    pub content: String,
}

pub struct ToolCall {
    pub call_id: String,
    pub name: String,
    pub arguments: String,
}

pub struct ToolCallResult {
    pub call_id: String,
    pub content: String,
}

impl LastMessage for Vec<Entry> {
    fn last_message(&mut self, role: Role) -> &mut Message {
        self.iter_mut()
            .rev()
            .find_map(|entry| match entry {
                Entry::Message(message) if message.role == role => Some(message),
                _ => None,
            })
            .unwrap_or_else(|| panic!("an message entry with role {:?} should exist", role))
    }
}

impl LastToolCall for Vec<Entry> {
    fn last_toolcall(&mut self) -> &mut ToolCall {
        self.iter_mut()
            .rev()
            .find_map(|entry| match entry {
                Entry::ToolCall(tool_call) => Some(tool_call),
                _ => None,
            })
            .expect("a tool call entry should exist")
    }
}

impl From<Message> for Entry {
    fn from(value: Message) -> Self {
        Entry::Message(value)
    }
}

impl From<ToolCall> for Entry {
    fn from(value: ToolCall) -> Self {
        Entry::ToolCall(value)
    }
}

impl From<ToolCallResult> for Entry {
    fn from(value: ToolCallResult) -> Self {
        Entry::ToolCallResult(value)
    }
}

impl Message {
    pub fn new(role: Role, content: String) -> Self {
        Self { role, content }
    }

    pub fn push_content(&mut self, delta: &str) {
        self.content.push_str(delta);
    }
}

impl From<OutputMessage> for Message {
    fn from(value: OutputMessage) -> Self {
        let content = value
            .content
            .into_iter()
            .map(|content| match content {
                OutputMessageContent::OutputText(output_text) => output_text.text,
                OutputMessageContent::Refusal(refusal) => refusal.refusal,
            })
            .fold(String::new(), |acc, v| acc + &v);

        Self {
            content,
            role: Role::Assistant,
        }
    }
}

impl ToolCall {
    pub fn push_arguments(&mut self, delta: &str) {
        self.arguments.push_str(delta);
    }
}

impl From<FunctionToolCall> for ToolCall {
    fn from(value: FunctionToolCall) -> Self {
        Self {
            call_id: value.call_id,
            name: value.name,
            arguments: value.arguments,
        }
    }
}

impl ToolCallResult {
    pub fn new(call_id: String, content: String) -> Self {
        Self { call_id, content }
    }
}
