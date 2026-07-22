use anyhow::Result;
use crossterm::event::{Event, KeyCode};

mod session;
mod terminal;
mod widget;

/// Session manager, dispatch and delegate work to current session.
#[derive(Default)]
pub struct App {
    session: Box<session::Session>,
}

impl App {
    pub fn run(&mut self) -> Result<()> {
        let mut terminal = terminal::init();

        loop {
            terminal.draw(|frame| {
                self.session.render(frame.area(), frame.buffer_mut());
            })?;

            match crossterm::event::read()? {
                Event::Key(key) if key.code == KeyCode::Esc => break,
                event => self.session.handle_event(event),
            };
        }

        terminal::restore();

        Ok(())
    }
}
