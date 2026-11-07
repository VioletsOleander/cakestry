use super::buffer::TextBuffer;

pub mod record;
pub mod scroll;

use record::Record;
use scroll::Scroll;

#[derive(Clone, Copy, Default)]
pub enum Mode {
    #[default]
    Prompt,
    Command,
    Scroll,
}

#[derive(Default)]
pub struct Session {
    pub system_prompt: String,
    pub records: Vec<Record>,
    pub scroll: Scroll,
    pub prompt_buffer: TextBuffer,
    pub mode: Mode,
    pub provider_index: usize,
    pub model_index: usize,
}

impl Session {
    pub fn new(system_prompt: String, provider_index: usize, model_index: usize) -> Self {
        Self {
            system_prompt,
            provider_index,
            model_index,
            ..Default::default()
        }
    }
}
