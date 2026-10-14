use anyhow::{Result, anyhow};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Paragraph, Widget as RatatuiWidget};

use super::Widget;
use crate::tui::chatstream::textwrap::wrap_line;

pub struct Prompt<'a> {
    lines: Vec<&'a str>,
    scroll: u16,
}

impl<'a> Prompt<'a> {
    pub fn build(prompt: &'a str, width: usize) -> Result<Self> {
        // 1 marker width + 1 space = 2
        let prefix_width = 2;

        match width.checked_sub(prefix_width) {
            Some(text_width) => Ok(Self {
                lines: prompt
                    .lines()
                    .flat_map(|line| wrap_line(line, text_width))
                    .collect(),
                scroll: 0,
            }),
            None => Err(anyhow!("failed to construct prompt view with given width")),
        }
    }
}

impl<'a> Widget for Prompt<'a> {
    fn scroll(self, scroll: u16) -> Self {
        Self {
            lines: self.lines,
            scroll,
        }
    }

    fn height(&self) -> usize {
        self.lines.len()
    }

    fn render(&self, area: Rect, frame: &mut Frame) {
        let [spans_area, paragraph_area] =
            Layout::horizontal([Constraint::Length(1), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);

        for i in self.scroll..spans_area.height {
            let span_area = Rect::new(spans_area.x, spans_area.y + i, spans_area.width, 1);

            if i == 0 {
                Span::styled(">", Style::default().fg(Color::Blue))
                    .render(span_area, frame.buffer_mut());
            } else {
                Span::styled(".", Style::default().fg(Color::Blue))
                    .render(span_area, frame.buffer_mut());
            }
        }

        Paragraph::new(self.lines.as_slice())
            .scroll((self.scroll, 0))
            .render(paragraph_area, frame.buffer_mut());
    }
}
