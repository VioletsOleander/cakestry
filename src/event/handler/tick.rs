use std::fmt::Write;

use anyhow::Result;

use crate::event::TickEvent;
use crate::state::{ResponsePhase, State};

pub struct TickEventHandler<'a> {
    icons: [&'a str; 10],
}

impl<'a> TickEventHandler<'a> {
    pub fn new() -> Self {
        Self {
            icons: ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
        }
    }

    pub fn handle(&self, event: TickEvent, state: &mut State) -> Result<()> {
        let index = event.count % self.icons.len();

        // We reuse the response field, to avoid implementing specific rendering logic for tick
        // line. Since the response content and tick frame are mutually exclusive, this kind of
        // reuse is ok.

        // The request submitter must have created a new exchange,
        // otherwise it is code logic error.
        let last_response = &mut state
            .exchanges
            .last_mut()
            .expect("there should be at least one existing exchange.")
            .response;

        match state.response_phase {
            ResponsePhase::Created => {
                last_response.clear();
                write!(last_response, "{} waiting", self.icons[index])?;
            }
            ResponsePhase::Reasoning => {
                last_response.clear();
                write!(last_response, "{} thinking", self.icons[index])?;
            }
            _ => (),
        }

        Ok(())
    }
}
