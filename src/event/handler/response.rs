use async_openai::error::OpenAIError;
use async_openai::types::responses::{OutputItem, ResponseStreamEvent, Role};
use crossbeam_channel::Sender;

use crate::event::TickStopEvent;
use crate::event::service::ToolService;
use crate::state::{ResponseStage, State};

pub struct ResponseEventHandler {
    sender: Sender<TickStopEvent>,
    tool_service: ToolService,
}

impl ResponseEventHandler {
    pub fn new(tool_service: ToolService, sender: Sender<TickStopEvent>) -> Self {
        Self {
            tool_service,
            sender,
        }
    }

    pub fn handle(&self, event: Result<ResponseStreamEvent, OpenAIError>, state: &mut State) {
        match event {
            Ok(event) => match event {
                ResponseStreamEvent::ResponseOutputItemAdded(event) => match event.item {
                    OutputItem::Message(_) => {
                        self.stop_tick();
                        state.resp_stage = ResponseStage::Answering;
                    }
                    OutputItem::Reasoning(_) => {
                        state.resp_stage = ResponseStage::Reasoning;
                    }
                    _ => (),
                },
                ResponseStreamEvent::ResponseOutputTextDelta(event) => {
                    state
                        .message_store
                        .stage_delta(&event.delta)
                        .expect("there should be a staged message");
                }
                ResponseStreamEvent::ResponseFunctionCallArgumentsDelta(event) => {}
                ResponseStreamEvent::ResponseFunctionCallArgumentsDone(event) => {}
                ResponseStreamEvent::ResponseCompleted(_) => {
                    state
                        .message_store
                        .commit_staged()
                        .expect("there should be a staged message");
                    state.resp_stage = ResponseStage::Finished;
                }
                ResponseStreamEvent::ResponseIncomplete(_) => {
                    state.message_store.abort_staged();
                    state.resp_stage = ResponseStage::Finished;
                }
                ResponseStreamEvent::ResponseFailed(_) => {
                    state.message_store.abort_staged();
                    state.resp_stage = ResponseStage::Finished;
                }
                _ => (),
            },
            Err(_) => {
                self.stop_tick();

                state.message_store.abort_staged();
                state.resp_stage = ResponseStage::Finished;
            }
        };
    }

    fn stop_tick(&self) {
        // If thread blocking `send` is used here, deadlock will possibly happen when the channel is
        // zero capacity: the main thread waiting for the tick thread to receive TickStopEvent, the
        // tick thread wait for the main thread to receive TickEvent.
        self.sender
            .try_send(TickStopEvent)
            .unwrap_or_else(|err| tracing::debug!("failed to send tick stop event due to {}", err));
    }
}
