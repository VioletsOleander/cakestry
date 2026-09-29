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
    exchanges: Vec<Exchange>,
    scroll_offset: usize,

    // Buffer.
    prompt_buffer: TextBuffer,
    command_buffer: TextBuffer,

    // Provider.
    providers: Vec<Provider>,
    provider_index: usize,

    // Others.
    mode: Mode,
    awaiting_response: bool,
    should_exit: bool,
}

impl State {
    pub fn build(config: Config) -> Result<Self> {
        let provider_index = config
            .providers()
            .iter()
            .position(|provider| provider.name() == config.provider())
            .ok_or_else(|| {
                anyhow!(
                    "failed to find configuration of provider '{}'",
                    config.provider()
                )
            })?;

        Ok(Self {
            exchanges: Vec::default(),
            scroll_offset: 0,
            prompt_buffer: TextBuffer::default(),
            command_buffer: TextBuffer::default(),
            providers: config.providers().to_vec(),
            provider_index,
            mode: Mode::Prompt,
            awaiting_response: false,
            should_exit: false,
        })
    }
}

// Special Setters.
impl State {
    pub fn increment_scroll_offset(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    pub fn decrement_scroll_offset(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }
}

// Normal Setters.
impl State {
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set_awaiting_response(&mut self, awaiting_response: bool) {
        self.awaiting_response = awaiting_response;
    }
}

// Special Getters.
impl State {
    pub fn active_buffer(&mut self) -> &mut TextBuffer {
        match self.mode {
            Mode::Prompt => &mut self.prompt_buffer,
            Mode::Command => &mut self.command_buffer,
        }
    }

    pub fn active_provider(&self) -> &Provider {
        &self.providers[self.provider_index]
    }
}

// Normal Getters.
impl State {
    pub fn exchanges(&self) -> &Vec<Exchange> {
        &self.exchanges
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn awaiting_response(&self) -> bool {
        self.awaiting_response
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    pub fn exchanges_mut(&mut self) -> &mut Vec<Exchange> {
        &mut self.exchanges
    }

    pub fn prompt_buffer_mut(&mut self) -> &mut TextBuffer {
        &mut self.prompt_buffer
    }

    pub fn command_buffer_mut(&mut self) -> &mut TextBuffer {
        &mut self.command_buffer
    }
}
