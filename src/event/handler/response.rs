use async_openai::error::OpenAIError;
use async_openai::types::responses::ResponseStreamEvent;

use crate::state::State;

pub struct ResponseEventHandler {}

impl ResponseEventHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle(&self, event: Result<ResponseStreamEvent, OpenAIError>, state: &mut State) {
        match event {
            Ok(event) => match event {
                ResponseStreamEvent::ResponseOutputTextDelta(event) => {
                    // The request submitter must have created a new exchange,
                    // otherwise it is code logic error.
                    let exchange = state
                        .exchanges
                        .last_mut()
                        .expect("There should be at least one existing exchange.");

                    exchange.response.push_str(&event.delta);
                }
                ResponseStreamEvent::ResponseCompleted(_) => state.awaiting_response = false,
                ResponseStreamEvent::ResponseIncomplete(_) => {
                    state.awaiting_response = false;
                    state.notification = "response incompelte".to_string();
                }
                ResponseStreamEvent::ResponseFailed(_) => {
                    state.awaiting_response = false;
                    state.notification = "response failed".to_string();
                }
                _ => (),
            },
            Err(err) => state.notification = err.to_string(),
        };
    }
}
