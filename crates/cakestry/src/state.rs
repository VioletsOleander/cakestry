use anyhow::{Result, anyhow};
use async_openai::types::responses::{FunctionToolArgs, Role, Tool};

use crate::config::{Config, Provider};

pub mod buffer;
pub mod message;

use buffer::TextBuffer;
use message::{Message, MessageSource};

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
    pub messages: Vec<Message>,
    pub stream_scroll: usize,
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
    pub tool_stage: ToolCallStage,
    pub resp_stage: ResponseStage,
    pub should_exit: bool,
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
        let messages = vec![Message::new(
            MessageSource::Role(Role::System),
            config.system_prompt,
        )];

        let provider_index = config
            .providers
            .iter()
            .position(|provider| provider.name == config.provider)
            .ok_or_else(|| {
                anyhow!(
                    "failed to find configuration of provider '{}'",
                    config.provider
                )
            })?;

        let tools = vec![Tool::Function(
            FunctionToolArgs::default()
                .name("test-tool")
                .build()
                .expect("name should be enough to build a function tool"),
        )];

        let tick_icons = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

        Ok(Self {
            messages,
            stream_scroll: 0,
            prompt_buffer: TextBuffer::default(),
            command_buffer: TextBuffer::default(),
            tick_icons,
            tick_index: 0,
            providers: config.providers,
            provider_index,
            tools,
            mode: Mode::Prompt,
            tool_stage: ToolCallStage::Finish,
            resp_stage: ResponseStage::Finish,
            should_exit: false,
        })
    }

    pub fn model(&self) -> &str {
        &self.providers[self.provider_index].model
    }
}
