use anyhow::{Result, anyhow};

use crate::config::{Config, Provider};

pub mod buffer;

use buffer::TextBuffer;

#[derive(Clone, Copy)]
pub enum Mode {
    Prompt,
    Command,
    Scroll,
}

#[derive(Clone, Copy)]
pub enum ResponsePhase {
    Created,
    Reasoning,
    Answering,
    Finished,
}

pub struct State {
    // Rendering and Storage.
    pub exchanges: Vec<Exchange>,
    pub stream_scroll: usize,
    pub notification: Notification,
    pub prompt_buffer: TextBuffer,
    pub command_buffer: TextBuffer,

    // Dynamic Configuration.
    pub system_prompt: String,
    pub providers: Vec<Provider>,
    pub provider_index: usize,

    // Operation Logic.
    pub mode: Mode,
    pub response_phase: ResponsePhase,
    pub should_exit: bool,
}

pub struct Exchange {
    pub prompt: String,
    pub response: String,
}

#[derive(Default)]
pub struct Notification {
    pub content: String,
}

impl State {
    pub fn build(config: Config) -> Result<Self> {
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

        Ok(Self {
            exchanges: Vec::default(),
            notification: Notification::default(),
            stream_scroll: 0,
            prompt_buffer: TextBuffer::default(),
            command_buffer: TextBuffer::default(),
            providers: config.providers,
            provider_index,
            mode: Mode::Prompt,
            system_prompt: config.system_prompt,
            response_phase: ResponsePhase::Finished,
            should_exit: false,
        })
    }
}

impl Exchange {
    pub fn new(prompt: String, response: String) -> Self {
        Self { prompt, response }
    }
}

impl Notification {
    pub fn set(&mut self, content: &str) {
        self.content.clear();
        self.content.push_str(content);
    }
}
