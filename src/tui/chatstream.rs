use std::cmp;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::state::State;

mod textwrap;
mod widget;

use widget::{Prompt, Response, SpaceLine, TextBuffer, Widget};

pub struct ChatStream;

/// Range in absolute y coordinates, with inclusive start and exclusive end.
#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl ChatStream {
    // The chatstream is allowed to have a height larger than area.height. The rendered viewport
    // is determined by the scroll offset and a part of the stream will be rendered to it.
    // This partial rendering idea applies to every scrollable widget in the chatstream.
    pub fn render(state: &State, area: Rect, frame: &mut Frame) {
        let view_range = Range::new(
            state.stream_scroll,
            state.stream_scroll + area.height as usize,
        );
        let mut widget_range = Range::new(0, 0);

        for exchange in &state.exchanges {
            if let Ok(prompt) = Prompt::build(&exchange.prompt, area.width as usize) {
                widget_range = widget_range.proceed().extend(prompt.height());
                try_render_widget(prompt, view_range, widget_range, area, frame);
            }

            {
                let spaceline = SpaceLine::default();
                widget_range = widget_range.proceed().extend(spaceline.height());
                try_render_widget(spaceline, view_range, widget_range, area, frame);
            }

            if let Ok(response) = Response::build(&exchange.response, area.width as usize) {
                widget_range = widget_range.proceed().extend(response.height());
                try_render_widget(response, view_range, widget_range, area, frame);
            }

            {
                let spaceline = SpaceLine::default();
                widget_range = widget_range.proceed().extend(spaceline.height());
                try_render_widget(spaceline, view_range, widget_range, area, frame);
            }
        }

        if let Ok(textbuffer) = TextBuffer::build(
            state.prompt_buffer.lines(),
            state.prompt_buffer.cursor(),
            area.width as usize,
        ) {
            widget_range = widget_range.proceed().extend(textbuffer.height());
            try_render_widget(textbuffer, view_range, widget_range, area, frame);
        }
    }
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Extend range end by `delta`.
    fn extend(self, delta: usize) -> Self {
        Self {
            start: self.start,
            end: self.end + delta,
        }
    }

    /// Set range start to range end.
    fn proceed(self) -> Self {
        Self {
            start: self.end,
            end: self.end,
        }
    }
}

fn try_render_widget(
    widget: impl Widget,
    view_range: Range,
    widget_range: Range,
    view_area: Rect,
    frame: &mut Frame,
) {
    if let Some(visible_range) = get_visible_range(view_range, widget_range) {
        let scroll = (visible_range.start - widget_range.start) as u16;
        let area = Rect::new(
            view_area.x,
            view_area.y + (visible_range.start - view_range.start) as u16,
            view_area.width,
            (visible_range.end - visible_range.start) as u16,
        );

        widget.scroll(scroll).render(area, frame);
    }
}

/// Compute and return the widget's visible range in viewport.
///
/// Return `None` if the widget is outside the viewport.
fn get_visible_range(viewport: Range, widget: Range) -> Option<Range> {
    if widget.end <= viewport.start || widget.start >= viewport.end {
        // Widget lies above or below the viewport.
        None
    } else {
        // Widget overlaps with the viewport.
        let start = cmp::max(viewport.start, widget.start);
        let end = cmp::min(viewport.end, widget.end);

        Some(Range::new(start, end))
    }
}
