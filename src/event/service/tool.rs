use std::thread;

use crate::event::ToolEvent;
use crate::state::State;

pub struct ToolService {}

impl ToolService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, state: &mut State) {}
}
