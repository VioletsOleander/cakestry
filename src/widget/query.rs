use crate::widget::{Angle, Dot, HeightMeasurable, Renderable, Scrollable};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget};

/// A widget to display user input text.
pub struct Query<'a> {
    lines: &'a [String],
    /// Scroll offset in y coordinate
    offset: u16,
}

impl<'a> Query<'a> {
    pub fn new(lines: &'a [String]) -> Self {
        Query {
            lines: lines,
            offset: 0,
        }
    }
}

impl<'a> Scrollable for Query<'a> {
    fn scroll(&mut self, offset: u16) {
        self.offset = offset;
    }
}

// TODO: Cache wrapped lines, might require very big refactor, like introducing stateful widget
impl<'a> HeightMeasurable for Query<'a> {
    fn height(&self, width: u16) -> usize {
        // span width + space width = 2
        if width < 2 {
            return 0;
        }

        let wrapped_lines = self
            .lines
            .iter()
            .flat_map(|line| textwrap::wrap(line, (width - 2) as usize));

        wrapped_lines.count()
    }
}

impl<'a> Renderable for Query<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [spans_area, paragraph_area] =
            Layout::horizontal([Constraint::Length(1), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);

        for i in 0..spans_area.height {
            if i < self.offset {
                continue;
            }

            let span_area = Rect::new(spans_area.x, spans_area.y + i, spans_area.width, 1);
            if i == 0 {
                Angle.render(span_area, buf);
            } else {
                Dot.render(span_area, buf);
            }
        }

        let wrapped_lines: Vec<_> = self
            .lines
            .iter()
            .flat_map(|line| textwrap::wrap(line, paragraph_area.width as usize))
            .map(Line::from)
            .collect();

        Paragraph::new(wrapped_lines)
            .scroll((self.offset, 0))
            .render(paragraph_area, buf);
    }
}
