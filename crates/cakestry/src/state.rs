use anyhow::{Context, Result};
use async_openai::types::responses::{
    EasyInputMessage, FunctionCallOutputItemParam, FunctionTool, FunctionToolCall, InputItem,
    InputParam, Item, Role, Tool as OpenAITool, WebSearchActionSearch, WebSearchActionSearchSource,
    WebSearchCallStatus, WebSearchTool, WebSearchToolCall, WebSearchToolCallAction,
};

use crate::config::{Config, Provider, Tool};

pub mod buffer;
pub mod session;

use buffer::TextBuffer;
use session::Session;

#[derive(Default)]
pub struct State {
    pub sessions: Vec<Session>,
    pub session_index: usize,
    pub command_buffer: TextBuffer,

    pub providers: Vec<Provider>,
    pub tools: Vec<Tool>,

    pub tick: usize,
    pub should_exit: bool,
}

impl State {
    pub fn build(config: Config) -> Result<Self> {
        let tools = config.tools.unwrap_or_default();
        let providers = config.providers;
        let sessions = vec![Session::new(
            config.system_prompt,
            providers
                .iter()
                .position(|v| v.name == config.default_provider)
                .with_context(|| {
                    format!(
                        "failed to find default provider {}",
                        config.default_provider
                    )
                })?,
        )];

        Ok(Self {
            sessions,
            providers,
            tools,
            ..Self::default()
        })
    }
}

impl State {

    pub fn input_param(&self) -> InputParam {
        let mut items: Vec<InputItem> = Vec::with_capacity(self..len());

        for entry in self.entries.iter() {
            match entry {
                Entry::SystemPrompt(content) => {
                    items.push(make_message(Role::System, content.clone()))
                }
                Entry::UserPrompt(content) => items.push(make_message(Role::User, content.clone())),
                Entry::AssistantResponse { stage, content } => {
                    match
                }
                Entry::AssistantResponse(content) => {
                    items.push(make_message(Role::Assistant, content.clone()))
                }
                Entry::ToolCall { id, kind, status } => match kind {
                    ToolKind::Local(local) => match local {
                        LocalTool::Function { name, arguments } => {
                            // Current context management strategy is to wait all lanuched tools to
                            // finish before sending the next request.
                            // Therefore by the time of making input params for next request, all
                            // tools, either remote or local should have finished execution.
                            let output = match status {
                                ToolStatus::Success { output } => output,
                                ToolStatus::Failure { error } => error,
                                ToolStatus::Running { .. } => {
                                    panic!("tool call status should be success or failure")
                                }
                            };

                            items.push(make_function_call(
                                id.clone(),
                                name.clone(),
                                arguments.clone(),
                            ));

                            items.push(make_function_output(
                                id.clone(),
                                name.clone(),
                                output.clone().unwrap_or_default(),
                            ));
                        }
                    },
                    ToolKind::Remote(remote) => match remote {
                        // The service will not return the raw web search tool call result, but
                        // instead return a summary in a form of response.
                        RemoteTool::WebSearch { queries, sources } => {
                            let status = match status {
                                ToolStatus::Success { .. } => WebSearchCallStatus::Completed,
                                ToolStatus::Failure { .. } => WebSearchCallStatus::Failed,
                                _ => {
                                    panic!(
                                        "web search tool call status should be success or failure"
                                    )
                                }
                            };

                            items.push(make_search_call(
                                id.clone(),
                                status,
                                queries.clone(),
                                sources.clone(),
                            ));
                        }
                    },
                },
            }
        }

        items.into()
    }
}



fn make_function_output(call_id: String, name: String, output: String) -> InputItem {
    Item::from(FunctionCallOutputItemParam {
        call_id: call_id.into(),
        name: name.into(),
        output: output.into(),
        id: None,
        namespace: None,
        status: None,
        caller: None,
    })
    .into()
}
