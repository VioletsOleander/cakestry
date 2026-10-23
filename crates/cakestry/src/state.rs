use std::cmp;

use anyhow::{Result, anyhow};
use async_openai::types::responses::{
    EasyInputContent, EasyInputMessage, FunctionCallOutput, FunctionCallOutputItemParam,
    FunctionTool, FunctionToolCall, InputItem, InputParam, Role, Tool,
};

use crate::config::{Config, Provider};

pub mod buffer;
pub mod entry;

use buffer::TextBuffer;
use entry::{Entry, Message};

#[derive(Clone, Copy)]
pub enum Mode {
    Prompt,
    Command,
    Scroll,
}

#[derive(Clone, Copy)]
pub enum ResponseStage {
    Create,
    Think,
    Answer,
    Finish,
}

pub enum ToolCallStage {
    Create,
    Finish,
}

pub struct State {
    // Rendering and Storage.
    pub entries: Vec<Entry>,
    pub scroll: Scroll,
    pub prompt_buffer: TextBuffer,
    pub command_buffer: TextBuffer,
    pub tick_icons: [&'static str; 10],
    pub tick_index: usize,

    // Dynamic Configuration.
    pub providers: Vec<Provider>,
    pub provider_index: usize,
    pub tools: Vec<Tool>,

    // Operation Logic.
    pub mode: Mode,
    pub toolcall_stage: ToolCallStage,
    pub response_stage: ResponseStage,
    pub should_exit: bool,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Scroll {
    pub value: usize,
    pub limit: usize,
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

impl State {
    pub fn build(config: Config) -> Result<Self> {
        let entries = vec![Message::new(Role::System, config.system_prompt).into()];

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
                .map(|tool| {
                    FunctionTool {
                        name: tool.name,
                        parameters: tool.parameters.map(|v| v.into()),
                        description: tool.description,
                        ..FunctionTool::default()
                    }
                    .into()
                })
                .collect(),
            None => Vec::new(),
        };

        let tick_icons = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

        Ok(Self {
            entries,
            scroll: Scroll::default(),
            prompt_buffer: TextBuffer::default(),
            command_buffer: TextBuffer::default(),
            tick_icons,
            tick_index: 0,
            providers,
            provider_index,
            tools,
            mode: Mode::Prompt,
            toolcall_stage: ToolCallStage::Finish,
            response_stage: ResponseStage::Finish,
            should_exit: false,
        })
    }

    pub fn model(&self) -> &str {
        &self.providers[self.provider_index].model
    }

    pub fn tick_icon(&self) -> &str {
        self.tick_icons[self.tick_index]
    }

    pub fn input_param(&self) -> InputParam {
        self.entries
            .iter()
            .map(|entry| match entry {
                Entry::Message(message) => EasyInputMessage {
                    role: message.role,
                    content: EasyInputContent::Text(message.content.clone()),
                    ..EasyInputMessage::default()
                }
                .into(),
                Entry::ToolCall(call) => InputItem::Item(
                    FunctionToolCall {
                        call_id: call.call_id.clone(),
                        name: call.name.clone(),
                        arguments: call.arguments.clone(),
                        id: None,
                        status: None,
                        namespace: None,
                    }
                    .into(),
                ),
                Entry::ToolCallResult(result) => InputItem::Item(
                    FunctionCallOutputItemParam {
                        call_id: result.call_id.clone(),
                        output: FunctionCallOutput::Text(result.content.clone()),
                        id: None,
                        status: None,
                    }
                    .into(),
                ),
            })
            .collect::<Vec<_>>()
            .into()
    }
}

impl Scroll {
    pub fn up(self, value: usize) -> Self {
        Self {
            value: self.value.saturating_sub(value),
            limit: self.limit,
        }
    }

    pub fn down(self, value: usize) -> Self {
        if self.value < self.limit {
            Self {
                value: cmp::min(self.value.saturating_add(value), value),
                limit: self.limit,
            }
        } else {
            self
        }
    }

    pub fn limit(self, limit: usize) -> Self {
        Self {
            value: self.value,
            limit,
        }
    }
}
