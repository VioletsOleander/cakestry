use anyhow::Result;
use crossterm::event::{EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use ratatui::DefaultTerminal;

mod session;

/// Session manager, dispatch and delegate work to current session
#[derive(Default)]
pub struct App {
    session: Box<session::Session>,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        execute!(std::io::stdout(), EnableMouseCapture)?;

        loop {
            terminal.draw(|frame| {
                self.session.render(frame.area(), frame.buffer_mut());
            })?;

            match crossterm::event::read()? {
                Event::Key(key) if key.code == KeyCode::Esc => break,
                event => self.session.handle_event(event),
            };
        }

        Ok(())
    }
}
