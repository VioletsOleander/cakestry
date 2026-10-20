use std::io::{self, Write, stderr, stdout};
use std::panic::{set_hook, take_hook};
use std::thread;

use anyhow::Result;
use crossbeam_channel::Sender;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, read};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct TerminalEventService;

impl TerminalEventService {
    pub fn build() -> Result<Self> {
        enable_raw_mode()?;
        execute!(stdout(), EnableMouseCapture)?;

        let current_hook = take_hook();
        set_hook(Box::new(move |info| {
            restore();
            current_hook(info);
        }));

        Ok(TerminalEventService)
    }

    pub fn run(&self, sender: Sender<Result<Event, io::Error>>) {
        thread::spawn(move || {
            loop {
                let result = read();

                if sender.send(result).is_err() {
                    // Technically, the break is not necessary. Because channel disconnection means
                    // receiver drop, which in turn means app termination.
                    // Therefore this thread will certainly be cleaned up even without this break.
                    // But to be pedantic, we can just keep it.
                    tracing::info!(
                        "terminal event channel disconnected, break the event reading loop"
                    );
                    break;
                }
            }
        });
    }
}

impl Drop for TerminalEventService {
    fn drop(&mut self) {
        restore();
    }
}

fn restore() {
    // Panic should be avoided in a panic hook, and in drop:
    // https://stackoverflow.com/questions/73467248/what-happens-when-a-panic-hook-panics

    if let Err(err) = disable_raw_mode() {
        tracing::error!("failed to disable raw mode, error {err} encountered");
        // Try write once, not forcing to empty the buffer, since the IO error is unknown.
        let _ = stderr().write("failed to disable raw mode, error {err} encountered\n".as_bytes());
    }

    if let Err(err) = execute!(stdout(), DisableMouseCapture) {
        tracing::error!("failed to disable mouse capture, error {err} encountered");
        let _ =
            stderr().write("failed to disable mouse capture, error {err} encountered\n".as_bytes());
    }
}
