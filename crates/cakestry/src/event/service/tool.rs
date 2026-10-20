use std::thread;

use crossbeam_channel::Sender;

use crate::event::ToolEvent;
use crate::state::State;

pub struct ToolEventService {
    sender: Sender<ToolEvent>,
}

impl ToolEventService {
    pub fn new(state: &State, sender: Sender<ToolEvent>) -> Self {
        Self { sender }
    }

    pub fn run(&self, name: String, arguments: String) {

    }
}
