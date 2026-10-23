use std::process::Command;
use std::thread;

use anyhow::{Context, anyhow};
use crossbeam_channel::Sender;
use serde_json::Value;

use crate::event::ToolEvent;

pub struct ToolEventService {
    sender: Sender<ToolEvent>,
}

impl ToolEventService {
    pub fn new(sender: Sender<ToolEvent>) -> Self {
        Self { sender }
    }

    pub fn run(&self, call_id: String, name: String, arguments: String) {
        let sender = self.sender.clone();

        thread::spawn(move || {
            match serde_json::from_str::<Value>(&arguments)
                .context("failed to parse arguments as valid json")
            {
                Ok(value) => match value.as_object() {
                    Some(map) => {
                        // Requirement for tool: accept no args or accept "--json <arguments>".
                        // If "arguments" is too large, the os limit on argv size may be hit. See
                        // man execve.
                        // For now, we do not consider it, if this situation happens in the future,
                        // consider classifying tools into two types: 1. use argv 2. use stdin (to
                        // avoid "arguments" too large to be passed in argv)
                        let result = if map.is_empty() {
                            Command::new(name).output()
                        } else {
                            Command::new(name).args(["--json", &arguments]).output()
                        };

                        match result.context("failed to start process") {
                            Ok(output) => {
                                send_event(sender, ToolEvent::Success { call_id, output })
                            }
                            Err(error) => send_event(sender, ToolEvent::Failure { call_id, error }),
                        };
                    }
                    None => {
                        send_event(
                            sender,
                            ToolEvent::Failure {
                                call_id,
                                error: anyhow!("faild to parse arguments as a valid json object"),
                            },
                        );
                    }
                },
                Err(error) => {
                    send_event(sender, ToolEvent::Failure { call_id, error });
                }
            };
        });
    }
}

fn send_event(sender: Sender<ToolEvent>, event: ToolEvent) {
    sender.send(event).unwrap_or_else(|err| {
        tracing::debug!(
            "tool event receiver unexpectedly disconnected due to {}",
            err
        )
    })
}
