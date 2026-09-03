use std::io;
use std::panic;
use std::thread;

use crossbeam_channel::Sender;
use crossterm::cursor::SetCursorStyle;
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event as CrosstermEvent, KeyCode as CrosstermKeyCode,
    KeyEvent as CrosstermKeyEvent, KeyModifiers as CrosstermKeyModifiers,
    MouseEvent as CrosstermMouseEvent,
};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::DefaultTerminal;
use ratatui::backend::CrosstermBackend;

use super::service::Service;
use super::session::Session;

mod render;

use render::TerminalRenderer;

pub type KeyCode = CrosstermKeyCode;
pub type KeyModifiers = CrosstermKeyModifiers;

pub enum TerminalEvent {
    Key(CrosstermKeyEvent),
    Mouse(CrosstermMouseEvent),
}

pub struct Terminal {
    default_terminal: DefaultTerminal,
}

impl Terminal {
    /// Draw the user interface in the terminal.
    pub fn draw(&mut self, session: &Session, service: &Service) {
        self.default_terminal
            .draw(|frame| TerminalRenderer::default().render(session, service, frame))
            .expect("The terminal should be able to render itself.");
    }

    /// Spawn a thread for infinite terminal event listening.
    ///
    /// The received event will be sent through `sender`.
    pub fn spawn_event_listener(&self, sender: Sender<TerminalEvent>) {
        thread::spawn(move || {
            loop {
                let crossterm_event = crossterm::event::read()
                    .expect("The terminal should have the capability to read crossterm events.");

                let terminal_event = match crossterm_event {
                    CrosstermEvent::Key(key) => TerminalEvent::Key(key),
                    CrosstermEvent::Mouse(mouse) => TerminalEvent::Mouse(mouse),
                    // Ignore not supported crossterm event.
                    _ => continue,
                };

                // If the channel is disconnected, break the event reading loop.
                // Technically, the break is not necessary. Because channel disconnection means
                // receiver drop, which in turn means app termination.
                // Therefore this thread will certainly be cleaned up even without this break.
                // But to be pedantic, we can just keep it.
                if sender.send(terminal_event).is_err() {
                    break;
                }
            }
        });
    }
}

impl Default for Terminal {
    fn default() -> Self {
        set_panic_hook();

        crossterm::terminal::enable_raw_mode()
            .expect("The terminal should have the capability to enable raw mode.");

        crossterm::execute!(
            io::stdout(),
            EnterAlternateScreen,
            EnableMouseCapture,
            SetCursorStyle::SteadyBar
        )
        .expect("The terminal should have the capability to execute given commands.");

        Self {
            default_terminal: DefaultTerminal::new(CrosstermBackend::new(io::stdout())).expect(
                "The terminal should have the capability to Initialize itself with crossbackend.",
            ),
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        restore_terminal();
    }
}

/// Set panic hook to restore the terminal before write panic info to stderr.
///
///
/// If panic hook is not set, the panic message will be written to the alternate buffer,
/// consequently not visible to the main buffer.
fn set_panic_hook() {
    let hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        restore_terminal();
        hook(info);
    }));
}

fn restore_terminal() {
    // Disabling raw mode first is important as it has more side effects than leaving the alternate
    // screen buffer.
    crossterm::terminal::disable_raw_mode()
        .expect("The terminal should have the capability to disable raw mode.");

    crossterm::execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        SetCursorStyle::DefaultUserShape
    )
    .expect("The terminal should have the capability to execute given commands.");
}
