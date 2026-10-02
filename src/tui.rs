use std::io::{Stdout, Write, stderr, stdout};
use std::panic::{set_hook, take_hook};

use anyhow::Result;
use crossterm::cursor::SetCursorStyle;
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};

use crate::state::State;

mod chatstream;
mod statusline;

use chatstream::ChatStream;
use statusline::StatusLine;

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn build() -> Result<Self> {
        execute!(stdout(), EnterAlternateScreen, SetCursorStyle::SteadyBar)?;

        let current_hook = take_hook();
        set_hook(Box::new(move |info| {
            restore();
            current_hook(info);
        }));

        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        Ok(Tui { terminal })
    }

    pub fn render(&mut self, state: &State) -> Result<()> {
        self.terminal.draw(|frame| {
            let [session_area, statusline_area] =
                Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());

            ChatStream::render(state, session_area, frame);
            StatusLine::render(state, statusline_area, frame);
        })?;

        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        restore();
    }
}

fn restore() {
    // Panic should be avoided in a panic hook, and in drop:
    // https://stackoverflow.com/questions/73467248/what-happens-when-a-panic-hook-panics

    // TUIRenderer should be initialized before TerminalEventService, so that it is dropped after TerminalEventService.
    // This makes sure that leaving alternate screen happens after disabling raw mode when exiting.
    if let Err(err) = execute!(
        stdout(),
        SetCursorStyle::DefaultUserShape,
        LeaveAlternateScreen
    ) {
        tracing::error!(
            "failed to restore cursor shape and leave alternate screen, error {err} encountered"
        );
        let _ = stderr().write(
            "failed to restore cursor shape and leave alternate screen, error {err} encountered\n"
                .as_bytes(),
        );
    }
}
