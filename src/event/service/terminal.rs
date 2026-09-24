use std::io::{self, Write, stderr, stdout};
use std::panic;
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

        let current_hook = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            restore();
            current_hook(info);
        }));

        Ok(TerminalEventService)
    }

    pub fn run(&self, sender: Sender<Result<Event, io::Error>>) -> Result<()> {
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

        Ok(())
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

    // Disabling raw mode first as it has more side effects than leaving the alternate screen buffer.
    if let Err(err) = disable_raw_mode() {
        tracing::error!("failed to disable raw mode, error {err} encountered");
        // Try write once, not forcing to empty the buffer, since the IO error is unkonwn.
        let _ = stderr().write("failed to disable raw mode, error {err} encountered\n".as_bytes());
    }

    if let Err(err) = execute!(stdout(), DisableMouseCapture) {
        tracing::error!("failed to disable mouse capture, error {err} encountered");
        let _ =
            stderr().write("failed to disable mouse capture, error {err} encountered\n".as_bytes());
    }
}
