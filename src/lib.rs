use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;

mod interface;
mod session;

#[derive(Default)]
pub struct App {
    session: Box<session::Session>,
    interface: interface::Interface,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            self.render(terminal)?;

            let exit = match crossterm::event::read()? {
                Event::Key(key) => self.handle_key(key),
                _ => false,
            };

            if exit {
                break;
            }
        }

        Ok(())
    }

    fn render(&self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal.draw(|frame| {
            self.interface
                .render(self.session.state(), frame.area(), frame.buffer_mut());
        })?;

        Ok(())
    }

    /// Return true if exiting app
    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => true,
            KeyCode::Enter => {
                let message = self.interface.pop_input();
                self.session.state_mut().add_message(dbg!(message));
                false
            }
            _ => {
                self.interface.handle_input(key);
                false
            }
        }
    }
}
