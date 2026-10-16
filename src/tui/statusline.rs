use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Span;
use ratatui::widgets::Widget;

use crate::state::{Mode, State};

pub struct StatusLine;

impl StatusLine {
    pub fn render(state: &State, area: Rect, frame: &mut Frame) {
        let model = &state.providers[state.provider_index].model;
        let model_span = Span::from(model);

        match state.mode {
            Mode::Command => {
                let command = state.command_buffer.content().unwrap_or_default();

                let mode_span = Span::from("Command:");
                let command_span = Span::from(command);

                let areas: [_; 3] = Layout::horizontal([
                    Constraint::Length(mode_span.width() as u16),
                    Constraint::Fill(1),
                    Constraint::Length(model_span.width() as u16),
                ])
                .areas(area);

                for (span, area) in [mode_span, command_span, model_span].iter().zip(areas) {
                    span.render(area, frame.buffer_mut());
                }
            }
            Mode::Prompt => {
                let mode_span = Span::from("Prompt");

                let areas: [_; 2] = Layout::horizontal([
                    Constraint::Fill(1),
                    Constraint::Length(model_span.width() as u16),
                ])
                .areas(area);

                for (span, area) in [mode_span, model_span].iter().zip(areas) {
                    span.render(area, frame.buffer_mut());
                }
            }
            Mode::Scroll => {
                let mode_span = Span::from("Scroll");

                let areas: [_; 2] = Layout::horizontal([
                    Constraint::Fill(1),
                    Constraint::Length(model_span.width() as u16),
                ])
                .areas(area);

                for (span, area) in [mode_span, model_span].iter().zip(areas) {
                    span.render(area, frame.buffer_mut());
                }
            }
        }
    }
}
