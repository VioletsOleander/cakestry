use anyhow::{Result, anyhow};

use crate::config::{Config, Provider};

pub mod buffer;
pub mod exchange;

use buffer::TextBuffer;
use exchange::Exchange;

#[derive(Clone, Copy)]
pub enum Mode {
    Prompt,
    Command,
}

pub struct State {
    // Render.
    pub exchanges: Vec<Exchange>,
    pub scroll_offset: usize,

    // Buffer.
    pub prompt_buffer: TextBuffer,
    pub command_buffer: TextBuffer,

    // Provider.
    pub providers: Vec<Provider>,
    pub provider_index: usize,

    // Others.
    pub mode: Mode,
    pub awaiting_response: bool,
    pub should_exit: bool,
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
            scroll_offset: 0,
            prompt_buffer: TextBuffer::default(),
            command_buffer: TextBuffer::default(),
            providers: config.providers,
            provider_index,
            mode: Mode::Prompt,
            awaiting_response: false,
            should_exit: false,
        })
    }
}
