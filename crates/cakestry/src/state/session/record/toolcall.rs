pub trait LastToolCall {
    /// Return the last tool call, panic if not found.
    fn last_toolcall(&mut self) -> &mut ToolCall;
}

pub enum ToolCall {
    Local,
    Remote,
}

pub struct LocalToolCall {

}

pub struct RemoteToolCall {

}
