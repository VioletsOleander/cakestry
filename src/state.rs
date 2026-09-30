use anyhow::{Result, anyhow};

use crate::config::{Config, Provider};

pub mod buffer;

use buffer::TextBuffer;

#[derive(Clone, Copy)]
pub enum Mode {
    Prompt,
    Command,
}

pub struct State {
    // Render.
    pub exchanges: Vec<Exchange>,
    pub notification: String,
    pub scroll_offset: usize,

    // Buffer.
    pub prompt_buffer: TextBuffer,
    pub command_buffer: TextBuffer,

    // Provider.
    pub system_prompt: String,
    pub providers: Vec<Provider>,
    pub provider_index: usize,

    // Others.
    pub mode: Mode,
    pub awaiting_response: bool,
    pub should_exit: bool,
}

pub struct Exchange {
    pub prompt: String,
    pub response: String,
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
            notification: String::new(),
            scroll_offset: 0,
            prompt_buffer: TextBuffer::default(),
            command_buffer: TextBuffer::default(),
            providers: config.providers,
            provider_index,
            mode: Mode::Prompt,
            system_prompt: config.system_prompt,
            awaiting_response: false,
            should_exit: false,
        })
    }
}

impl Exchange {
    pub fn new(prompt: String, response: String) -> Self {
        Self { prompt, response }
    }
}
