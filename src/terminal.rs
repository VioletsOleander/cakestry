//! Utility to initialize and deinitialize an [`DefaultTerminal`] instance.
//!
//! This file is adapted from `ratatui/src/init.rs`

use std::io::{Result, stdout};
use std::panic;

use crossterm::cursor::SetCursorStyle;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::backend::CrosstermBackend;
use ratatui::{DefaultTerminal, Terminal};

/// Initialize and return a `DefaultTerminal` instance.
pub fn init() -> DefaultTerminal {
    try_init().expect("failed to initialize terminal")
}

/// Restore the terminal to its original state.
pub fn restore() {
    if let Err(err) = try_restore() {
        // There's not much we can do if restoring the terminal fails, so we just print the error.
        eprintln!("Failed to restore terminal: {err}");
    }
}

fn try_init() -> Result<DefaultTerminal> {
    set_panic_hook();
    enable_raw_mode()?;

    execute!(
        stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
        SetCursorStyle::SteadyBar
    )?;

    Terminal::new(CrosstermBackend::new(stdout()))
}

fn try_restore() -> Result<()> {
    // Disabling raw mode first is important as it has more side effects than leaving the alternate
    // screen buffer.
    disable_raw_mode()?;

    execute!(
        stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        SetCursorStyle::DefaultUserShape
    )?;

    Ok(())
}

fn set_panic_hook() {
    let hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        restore();
        hook(info);
    }));
}
