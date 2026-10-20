use async_openai::error::OpenAIError;
use async_openai::types::responses::{OutputItem, ResponseStreamEvent, Role};
use crossbeam_channel::Sender;

use crate::event::TickStopEvent;
use crate::event::service::{TickEventService, ToolEventService};
use crate::state::message::{Message, MessageSource};
use crate::state::{ResponseStage, State, ToolCallStage};

pub struct ResponseEventHandler<'a> {
    sender: Sender<TickStopEvent>,
    tick_service: &'a TickEventService,
    tool_service: ToolEventService,
}

impl<'a> ResponseEventHandler<'a> {
    pub fn new(
        tool_service: ToolEventService,
        tick_service: &'a TickEventService,
        sender: Sender<TickStopEvent>,
    ) -> Self {
        Self {
            tool_service,
            tick_service,
            sender,
        }
    }

    pub fn handle(&self, event: Result<ResponseStreamEvent, OpenAIError>, state: &mut State) {
        // A stream should either ends with item message + tool call or tool call. Message item is
        // supposed to never appear after a tool call item.
        match event {
            Ok(event) => match event {
                ResponseStreamEvent::ResponseOutputItemAdded(event) => match event.item {
                    OutputItem::Reasoning(_) => {
                        state.resp_stage = ResponseStage::Think;
                    }
                    OutputItem::Message(_) => {
                        self.send_tickstop();

                        state.messages.push(Message::new(
                            MessageSource::Role(Role::Assistant),
                            String::new(),
                        ));
                        state.resp_stage = ResponseStage::Answer;
                    }
                    OutputItem::FunctionCall(call) => {
                        self.send_tickstop();

                        state.messages.push(Message::new(
                            MessageSource::Tool {
                                call_id: call.call_id,
                            },
                            String::new(),
                        ));
                        state.resp_stage = ResponseStage::Answer;
                    }
                    _ => (),
                },
                ResponseStreamEvent::ResponseOutputTextDelta(event) => {
                    // Last message retrieval failure is code logic error therefore is
                    // unrecoverable.
                    state
                        .messages
                        .last_mut()
                        .expect("there should be a assistant message in the last")
                        .push_str(&event.delta);
                }
                ResponseStreamEvent::ResponseOutputItemDone(event) => match event.item {
                    OutputItem::FunctionCall(call) => {
                        let name = call.name;
                        let arguments = call.arguments;

                        self.tool_service.run(name, arguments);
                    }
                    _ => (),
                },
                ResponseStreamEvent::ResponseCompleted(_) => {
                    state.resp_stage = ResponseStage::Finish;
                }
                ResponseStreamEvent::ResponseIncomplete(_) => {
                    state.resp_stage = ResponseStage::Finish;
                }
                ResponseStreamEvent::ResponseFailed(_) => {
                    state.resp_stage = ResponseStage::Finish;
                }
                _ => (),
            },
            Err(_) => {
                self.send_tickstop();

                state.resp_stage = ResponseStage::Finish;
            }
        };
    }

    fn send_tickstop(&self) {
        // If thread blocking `send` is used here, deadlock will possibly happen when the channel is
        // zero capacity: the main thread waiting for the tick thread to receive TickStopEvent, the
        // tick thread wait for the main thread to receive TickEvent.
        self.sender
            .try_send(TickStopEvent)
            .unwrap_or_else(|err| tracing::debug!("failed to send tick stop event due to {}", err));
    }
}
