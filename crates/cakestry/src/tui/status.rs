use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Span;
use ratatui::widgets::Widget;

use crate::state::{Mode, State};

mod textspan;

use textspan::TextSpan;

pub struct StatusLine;

impl StatusLine {
    pub fn render(state: &State, area: Rect, frame: &mut Frame) {
        let buf = frame.buffer_mut();

        let model = &state.providers[state.provider_index].model;

        let name_span = Span::from(state.mode.name());
        let model_span = Span::from(model);

        let [name_area, model_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(model_span.width() as u16),
        ])
        .areas(area);

        if matches!(state.mode, Mode::Command) {
            let command_span = TextSpan::new(
                state
                    .command_buffer
                    .line(0)
                    .map_or("", |line| line.as_str()),
                state.command_buffer.cursor().char,
            );

            let [name_area, command_area] = Layout::horizontal([
                Constraint::Length(name_span.width() as u16),
                Constraint::Fill(1),
            ])
            .areas(area);

            name_span.render(name_area, buf);
            command_span.render(command_area, buf);
            model_span.render(model_area, buf);

            if let Some(position) = command_span.compute_cursor_position(command_area) {
                frame.set_cursor_position(position);
            }
        } else {
            name_span.render(name_area, buf);
            model_span.render(model_area, buf);
        }
    }
}
