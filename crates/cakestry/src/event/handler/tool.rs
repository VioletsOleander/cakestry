use crate::event::ToolEvent;
use crate::event::service::{ResponseEventService, TickEventService};
use crate::state::entry::ToolCallResult;
use crate::state::{ResponseStage, State, ToolCallStage};

pub struct ToolEventHandler<'a> {
    resp_service: &'a ResponseEventService,
    tick_service: &'a TickEventService,
}

impl<'a> ToolEventHandler<'a> {
    pub fn new(resp_service: &'a ResponseEventService, tick_service: &'a TickEventService) -> Self {
        Self {
            resp_service,
            tick_service,
        }
    }

    pub fn handle(&self, event: ToolEvent, state: &mut State) {
        self.stop_tick(state);

        state.entries.push(make_result(event).into());
        state.toolcall_stage = ToolCallStage::Finish;

        self.resp_service.create_responses(
            state.input_param(),
            state.tools.clone(),
            state.model().to_string(),
        );
        self.tick_service.run();

        state.response_stage = ResponseStage::Create;
    }

    fn stop_tick(&self, state: &State) {
        if matches!(state.toolcall_stage, ToolCallStage::Create) {
            self.tick_service.stop();
        }
    }
}

fn make_result(event: ToolEvent) -> ToolCallResult {
    match event {
        ToolEvent::Success { call_id, output } => {
            let content = if output.status.success() {
                match String::from_utf8(output.stdout) {
                    Ok(stdout) => stdout,
                    // from_utf8_lossy will insert signs to replace invalid characters, so clone
                    // must be used here.
                    Err(err) => String::from_utf8_lossy(err.as_bytes()).into_owned(),
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);

                match output.status.code() {
                    Some(code) => {
                        format!(
                            "error: process exited with code {}, with stderr {}",
                            code, stderr
                        )
                    }
                    None => {
                        format!(
                            "error: process terminated by signal, with stderr {}",
                            stderr
                        )
                    }
                }
            };

            ToolCallResult::new(call_id, content)
        }
        ToolEvent::Failure { call_id, error } => ToolCallResult::new(call_id, error.to_string()),
    }
}
