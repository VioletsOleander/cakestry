use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;

use crate::state::{Mode, State};

pub struct StatusLine;

impl StatusLine {
    pub fn render(state: &State, area: Rect, frame: &mut Frame) {
        match state.mode {
            Mode::Command => {
                let command = state.command_buffer.content().unwrap_or_default();
                let command_span = Span::from(command);

                let model = &state.providers[state.provider_index].model;
                let model_span = Span::from(model);

                let [command_area, model_area] = Layout::horizontal([
                    Constraint::Fill(1),
                    Constraint::Length(model_span.width() as u16),
                ])
                .areas(area);

                command_span.render(command_area, frame.buffer_mut());
                model_span.render(model_area, frame.buffer_mut());
            }
            Mode::Prompt => {
                let model = &state.providers[state.provider_index].model;
                let line = Line::from(model.as_str()).right_aligned();

                line.render(area, frame.buffer_mut());
            }
        };
    }
}
