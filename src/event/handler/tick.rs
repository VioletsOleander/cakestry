use crate::event::TickEvent;
use crate::state::State;

pub struct TickEventHandler {}

impl TickEventHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle(&self, event: TickEvent, state: &mut State) {
        state.tick_index = event.count % state.tick_icons.len();
    }
}
