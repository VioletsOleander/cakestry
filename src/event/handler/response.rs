use async_openai::error::OpenAIError;
use async_openai::types::responses::{OutputItem, ResponseStreamEvent};
use crossbeam_channel::Sender;

use crate::event::TickStopEvent;
use crate::state::{ResponsePhase, State};

pub struct ResponseEventHandler {
    sender: Sender<TickStopEvent>,
}

impl ResponseEventHandler {
    pub fn new(sender: Sender<TickStopEvent>) -> Self {
        Self { sender }
    }

    pub fn handle(&self, event: Result<ResponseStreamEvent, OpenAIError>, state: &mut State) {
        match event {
            Ok(event) => match event {
                ResponseStreamEvent::ResponseOutputItemAdded(event) => match event.item {
                    OutputItem::Message(_) => {
                        self.try_stop_tick();
                        self.get_last_response(state).clear();

                        state.response_phase = ResponsePhase::Answering;
                    }
                    OutputItem::Reasoning(_) => {
                        state.response_phase = ResponsePhase::Reasoning;
                    }
                    _ => (),
                },
                ResponseStreamEvent::ResponseOutputTextDelta(event) => {
                    self.get_last_response(state).push_str(&event.delta);
                }
                ResponseStreamEvent::ResponseCompleted(_) => {
                    state.response_phase = ResponsePhase::Finished;
                }
                ResponseStreamEvent::ResponseIncomplete(_) => {
                    state.response_phase = ResponsePhase::Finished;
                    state.notification.set("response incompelte");
                }
                ResponseStreamEvent::ResponseFailed(_) => {
                    state.response_phase = ResponsePhase::Finished;
                    state.notification.set("response failed");
                }
                _ => (),
            },
            Err(err) => {
                self.try_stop_tick();

                state.response_phase = ResponsePhase::Finished;
                state.notification.set(err.to_string().as_str());
            }
        };
    }

    fn get_last_response<'a>(&self, state: &'a mut State) -> &'a mut String {
        // The request submitter must have created a new exchange,
        // otherwise it is code logic error.
        &mut state
            .exchanges
            .last_mut()
            .expect("there should be at least one existing exchange.")
            .response
    }

    fn try_stop_tick(&self) {
        // If thread blocking `send` is used here, deadlock will possibly happen: the main thread
        // waiting for the tick thread to receive TickStopEvent, the tick thread wait for the main
        // thread to receive TickEvent.
        self.sender
            .try_send(TickStopEvent)
            .unwrap_or_else(|err| tracing::debug!("failed to send tick stop event due to {}", err));
    }
}
