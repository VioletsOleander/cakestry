use anyhow::Result;

pub mod handler;
pub mod service;

pub struct ToolEvent {
    result: Result<String>,
}

#[derive(Debug)]
pub struct TickEvent {
    count: usize,
}

pub struct TickStopEvent;

impl TickEvent {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}
