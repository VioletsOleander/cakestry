use anyhow::{Result, anyhow};
use async_openai::types::responses::{
    EasyInputMessage, FunctionCallOutputItemParam, FunctionTool, FunctionToolCall, InputItem,
    InputParam, Item, Role, Tool as OpenAITool, WebSearchActionSearch, WebSearchActionSearchSource,
    WebSearchCallStatus, WebSearchTool, WebSearchToolCall, WebSearchToolCallAction,
};

use crate::config::{Config, Provider, Tool};

pub mod buffer;
pub mod entry;
pub mod scroll;

use buffer::TextBuffer;
use entry::{Entry, LocalTool, RemoteTool, ToolKind, ToolStatus};
use scroll::Scroll;

#[derive(Clone, Copy, Default)]
pub enum Mode {
    #[default]
    Prompt,
    Command,
    Scroll,
}

#[derive(Clone, Copy, Default)]
pub enum Turn {
    #[default]
    User,
    Assistant,
}

#[derive(Default)]
pub struct State {
    // Rendering and Storage.
    pub entries: Vec<Entry>,
    pub scroll: Scroll,
    pub prompt_buffer: TextBuffer,
    pub command_buffer: TextBuffer,
    pub tick_icons: [&'static str; 10],

    // Dynamic Configuration.
    pub providers: Vec<Provider>,
    pub provider_index: usize,
    pub tools: Vec<OpenAITool>,

    // Operation Logic.
    pub mode: Mode,
    pub turn: Turn,
    pub should_exit: bool,
}

impl State {
    pub fn build(config: Config) -> Result<Self> {
        let entries = vec![Entry::SystemPrompt(config.system_prompt)];

        let providers = config.providers;
        let provider_index = providers
            .iter()
            .position(|provider| provider.name == config.provider)
            .ok_or_else(|| {
                anyhow!(
                    "failed to find configuration of provider '{}'",
                    config.provider
                )
            })?;

        let tools = match config.tools {
            Some(tools) => tools
                .into_iter()
                .map(|tool| match tool {
                    Tool::Function {
                        name,
                        parameters,
                        description,
                    } => FunctionTool {
                        name,
                        parameters: parameters.map(|v| v.into()),
                        description,
                        ..FunctionTool::default()
                    }
                    .into(),
                    Tool::WebSearch => WebSearchTool::default().into(),
                })
                .collect(),
            None => Vec::new(),
        };
        let tick_icons = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

        Ok(Self {
            entries,
            tick_icons,
            providers,
            provider_index,
            tools,
            ..Self::default()
        })
    }

    pub fn model(&self) -> &str {
        &self.providers[self.provider_index].model
    }

    pub fn tick_icon(&self, index: usize) -> &str {
        self.tick_icons[index]
    }

    pub fn input_param(&self) -> InputParam {
        let mut items: Vec<InputItem> = Vec::with_capacity(self.entries.len());

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

impl Mode {
    pub fn name(&self) -> &str {
        match self {
            Mode::Prompt => "Prompt",
            Mode::Command => "Command",
            Mode::Scroll => "Scroll",
        }
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


