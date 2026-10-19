use crate::event::ToolEvent;
use crate::state::State;

pub struct ToolEventHandler {}

impl ToolEventHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle(&self, event: ToolEvent, state: &mut State) {
        todo!()
    }
}
