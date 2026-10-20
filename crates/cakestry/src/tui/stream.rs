use std::cmp;

use async_openai::types::responses::Role;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use crate::state::message::{Message, MessageSource};
use crate::state::{Mode, ResponseStage, State, ToolCallStage};

mod textwrap;
mod widget;

use widget::{Paragraph, Prompt, Separator, TextBuffer, Widget};

pub struct MessageStream;

/// Range in absolute y coordinates, with inclusive start and exclusive end.
#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

struct Viewport {
    scroll: u16,
    area: Rect,
}

impl MessageStream {
    // The message stream is allowed to have a height larger than area.height. The rendered viewport
    // is determined by the scroll offset and a part of the stream will be rendered to it. This
    // partial rendering idea applies to every scrollable widget in the message stream.
    pub fn render(state: &State, area: Rect, frame: &mut Frame) {
        let buf = frame.buffer_mut();

        let view = Range::new(
            state.stream_scroll,
            state.stream_scroll + area.height as usize,
        );
        let mut window = Range::new(0, 0);

        for message in &state.messages {
            window = render_message(message, view, window, area, buf);
        }

        let tick_content = match state.resp_stage {
            ResponseStage::Create => Some(format!(
                "{} {}",
                state.tick_icons[state.tick_index], "thinking"
            )),
            ResponseStage::Think => Some(format!(
                "{} {}",
                state.tick_icons[state.tick_index], "thinking"
            )),
            _ => None,
        };

        if let Some(tick_content) = tick_content
            && let Ok(widget) = Paragraph::build(&tick_content, area.width as usize)
        {
            window = render_widget(widget, view, window, area, buf);
            window = render_widget(Separator::new(), view, window, area, buf);
        }

        if let Ok(textbuffer) = TextBuffer::build(
            state.prompt_buffer.lines(),
            state.prompt_buffer.cursor(),
            area.width as usize,
        ) {
            window = window.proceed().extend(textbuffer.height());

            if let Some(viewport) = get_viewport(view, window, area) {
                let textbuffer = textbuffer.scroll(viewport.scroll);

                textbuffer.render(viewport.area, buf);

                if matches!(state.mode, Mode::Prompt)
                    && let Some(position) = textbuffer.compute_cursor_position(viewport.area)
                {
                    frame.set_cursor_position(position);
                }
            }
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

fn render_message(
    message: &Message,
    view: Range,
    window: Range,
    area: Rect,
    buf: &mut Buffer,
) -> Range {
    let mut window = window;

    match message.source {
        MessageSource::Role(Role::User) => {
            if let Ok(widget) = Prompt::build(&message.content, area.width as usize) {
                window = render_widget(widget, view, window, area, buf);
                window = render_widget(Separator::new(), view, window, area, buf);
            }
        }
        MessageSource::Role(Role::Assistant) | MessageSource::Tool { .. } => {
            if let Ok(widget) = Paragraph::build(&message.content, area.width as usize) {
                window = render_widget(widget, view, window, area, buf);
                window = render_widget(Separator::new(), view, window, area, buf);
            }
        }
        _ => (),
    };

    window
}

fn render_widget(
    widget: impl Widget,
    view: Range,
    window: Range,
    area: Rect,
    buf: &mut Buffer,
) -> Range {
    let window = window.proceed().extend(widget.height());

    if let Some(viewport) = get_viewport(view, window, area) {
        widget.scroll(viewport.scroll).render(viewport.area, buf);
    }

    window
}

fn get_viewport(view_range: Range, widget_range: Range, view_area: Rect) -> Option<Viewport> {
    if let Some(visible_range) = get_visible_range(view_range, widget_range) {
        let scroll = (visible_range.start - widget_range.start) as u16;
        let area = Rect::new(
            view_area.x,
            view_area.y + (visible_range.start - view_range.start) as u16,
            view_area.width,
            (visible_range.end - visible_range.start) as u16,
        );

        Some(Viewport { scroll, area })
    } else {
        None
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
