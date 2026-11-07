use anyhow::{Context, Result};

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

        let (provider_index, provider) = providers
            .iter()
            .enumerate()
            .find(|(_, v)| v.name == config.default_provider)
            .with_context(|| {
                format!(
                    "failed to find default provider {} in providers {:?}",
                    config.default_provider, providers
                )
            })?;
        let model_index = provider
            .models
            .iter()
            .position(|v| v == &provider.default_model)
            .with_context(|| {
                format!(
                    "failed to find default model {} in provider {:?}",
                    provider.default_model, provider
                )
            })?;

        let sessions = vec![Session::new(
            config.system_prompt,
            provider_index,
            model_index,
        )];

        Ok(Self {
            sessions,
            providers,
            tools,
            ..Default::default()
        })
    }

    pub fn session(&mut self) -> &mut Session {
        &mut self.sessions[self.session_index]
    }
}
