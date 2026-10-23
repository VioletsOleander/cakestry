use async_openai::error::OpenAIError;
use async_openai::types::responses::{OutputItem, ResponseStreamEvent, Role};

use crate::event::service::{TickEventService, ToolEventService};
use crate::state::entry::{LastMessage, LastToolCall, Message, ToolCall};
use crate::state::{ResponseStage, State, ToolCallStage};

pub struct ResponseEventHandler<'a> {
    tick_service: &'a TickEventService,
    tool_service: ToolEventService,
}

impl<'a> ResponseEventHandler<'a> {
    pub fn new(tool_service: ToolEventService, tick_service: &'a TickEventService) -> Self {
        Self {
            tool_service,
            tick_service,
        }
    }

    pub fn handle(&self, event: Result<ResponseStreamEvent, OpenAIError>, state: &mut State) {
        // A stream should either ends with item message + tool call or tool call. Message item is
        // supposed to never appear after a tool call item.
        match event {
            Ok(event) => self.handle_event(event, state),
            Err(error) => self.handle_error(error, state),
        };
    }

    fn handle_event(&self, event: ResponseStreamEvent, state: &mut State) {
        match event {
            ResponseStreamEvent::ResponseOutputItemAdded(event) => match event.item {
                OutputItem::Reasoning(_) => {
                    state.response_stage = ResponseStage::Think;
                }
                OutputItem::Message(message) => {
                    self.stop_tick(state);

                    state.entries.push(Message::from(message).into());
                    state.response_stage = ResponseStage::Answer;
                }
                OutputItem::FunctionCall(call) => {
                    self.stop_tick(state);

                    state.entries.push(ToolCall::from(call).into());
                    state.response_stage = ResponseStage::Answer;
                }
                _ => (),
            },
            ResponseStreamEvent::ResponseOutputTextDelta(event) => {
                state
                    .entries
                    .last_message(Role::Assistant)
                    .push_content(&event.delta);
            }
            ResponseStreamEvent::ResponseFunctionCallArgumentsDelta(event) => {
                state.entries.last_toolcall().push_arguments(&event.delta);
            }
            ResponseStreamEvent::ResponseOutputItemDone(event) => {
                if let OutputItem::FunctionCall(call) = event.item {
                    self.tool_service
                        .run(call.call_id, call.name, call.arguments);
                    self.tick_service.run();

                    state.response_stage = ResponseStage::Finish;
                    state.toolcall_stage = ToolCallStage::Create;
                }
            }
            ResponseStreamEvent::ResponseCompleted(_) => {
                state.response_stage = ResponseStage::Finish;
            }
            ResponseStreamEvent::ResponseIncomplete(_) => {
                state.response_stage = ResponseStage::Finish;
            }
            ResponseStreamEvent::ResponseFailed(_) => {
                state.response_stage = ResponseStage::Finish;
            }
            _ => (),
        }
    }

    fn handle_error(&self, _error: OpenAIError, state: &mut State) {
        self.stop_tick(state);
        state.response_stage = ResponseStage::Finish;
    }

    fn stop_tick(&self, state: &State) {
        // Match response stage to avoid sending TickStopEvent multiple times.
        if matches!(
            state.response_stage,
            ResponseStage::Create | ResponseStage::Think
        ) {
            self.tick_service.stop();
        }
    }
}
