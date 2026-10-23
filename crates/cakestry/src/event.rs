use std::process::Output;

use anyhow::Error;

pub mod handler;
pub mod service;

pub enum ToolEvent {
    Success { call_id: String, output: Output },
    Failure { call_id: String, error: Error },
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
