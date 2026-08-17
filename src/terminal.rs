use std::io;
use std::panic;

use crossterm::cursor::SetCursorStyle;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};
use ratatui::{DefaultTerminal, Frame};

use super::service::Service;
use super::session::Session;

mod document;
mod statusline;

/// Wrapper of [`DefaultTerminal`].
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

    /// Read [`Event`] from user interaction and return it.
    pub fn read_event(&self) -> Event {
        crossterm::event::read()
            .expect("The terminal should have the capability to read events from crossterm.")
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

#[derive(Default)]
struct TerminalRenderer {
    document_offset: usize,
    document_view: (usize, usize),
}

impl TerminalRenderer {
    /// Render the visible part of the document and a status line on the given `frame`.
    ///
    /// Exchanges and textarea of `session` will be rendered on a virtual document, which has larger
    /// length than the viewport of `frame`. The actually rendered area starts from `session`'s
    /// scroll offset.
    pub fn render(&mut self, session: &Session, service: &Service, frame: &mut Frame) {
        let [document_area, statusline_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());

        self.render_document(session, document_area, frame);
        self.render_statusline(service, statusline_area, frame.buffer_mut());
    }
}
