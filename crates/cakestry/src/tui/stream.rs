use std::cmp;

use async_openai::types::responses::Role;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use crate::state::entry::{Entry, Message, ToolCall, ToolCallResult};
use crate::state::{Mode, ResponseStage, State, ToolCallStage};

mod textwrap;
mod widget;

use textwrap::wrap_line;
use widget::{Paragraph, Prompt, Scroll, Separator};

const MIN_WIDTH: u16 = 3;

pub struct WidgetStream;

/// Range in absolute y coordinates, with inclusive start and exclusive end.
#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

struct RangeContext {
    viewport: Range,
    widget: Range,
}

impl WidgetStream {
    pub fn render(state: &mut State, area: Rect, frame: &mut Frame) {
        if area.width < MIN_WIDTH {
            return;
        }

        let buf = frame.buffer_mut();
        let mut context = RangeContext::new(state, area);

        for entry in &state.entries {
            match entry {
                Entry::Message(message) => render_message(message, &mut context, area, buf),
                Entry::ToolCall(call) => render_toolcall(call, &mut context, area, buf),
                Entry::ToolCallResult(result) => {
                    render_toolcall_result(result, &mut context, area, buf)
                }
            }
        }

        render_tick(state, &mut context, area, buf);
        render_prompt_buffer(state, &mut context, area, frame);

        state.scroll = state.scroll.limit(context.widget.start);
    }
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl RangeContext {
    fn new(state: &State, area: Rect) -> Self {
        Self {
            viewport: Range::new(
                state.scroll.value,
                state.scroll.value + area.height as usize,
            ),
            widget: Range::new(0, 0),
        }
    }

    /// Return (scroll, offset, height) or None.
    fn params(&self) -> Option<(u16, u16, u16)> {
        get_visible_range(self.viewport, self.widget).map(|visible| {
            (
                (visible.start - self.widget.start) as u16,
                (visible.start - self.viewport.start) as u16,
                (visible.end - visible.start) as u16,
            )
        })
    }

    fn offset(&mut self, delta: usize) -> &Self {
        self.widget.start = self.widget.end;
        self.widget.end += delta;

        self
    }
}

fn render_message(message: &Message, context: &mut RangeContext, area: Rect, buf: &mut Buffer) {
    match message.role {
        Role::User => {
            render_prompt_paragraph(&message.content, Color::Blue, context, area, buf);
            render_separator(context, area, buf);
        }
        Role::Assistant => {
            render_paragraph(&message.content, context, area, buf);
            render_separator(context, area, buf);
        }
        _ => (),
    }
}

fn render_toolcall(call: &ToolCall, context: &mut RangeContext, area: Rect, buf: &mut Buffer) {
    let content = format!("Call: {} with arguments {}", call.name, call.arguments);

    render_paragraph(&content, context, area, buf);
    render_separator(context, area, buf);
}

fn render_toolcall_result(
    result: &ToolCallResult,
    context: &mut RangeContext,
    area: Rect,
    buf: &mut Buffer,
) {
    let content = format!("Result: {}", result.content);

    render_paragraph(&content, context, area, buf);
    render_separator(context, area, buf);
}

fn render_tick(state: &State, context: &mut RangeContext, area: Rect, buf: &mut Buffer) {
    if let Some(content) = match state.response_stage {
        ResponseStage::Create => Some(format!("{} waiting", state.tick_icon())),
        ResponseStage::Think => Some(format!("{} thinking", state.tick_icon())),
        _ => match state.toolcall_stage {
            ToolCallStage::Create => Some(format!("{} calling", state.tick_icon())),
            _ => None,
        },
    } {
        render_paragraph(&content, context, area, buf);
        render_separator(context, area, buf);
    }
}

fn render_prompt_buffer(state: &State, context: &mut RangeContext, area: Rect, frame: &mut Frame) {
    let width = area.width as usize - 2;
    let line_wraps: Vec<_> = state
        .prompt_buffer
        .lines()
        .iter()
        .map(|line| wrap_line(line, width))
        .collect();

    // `try_fold` must return Err, otherwise it means the buffer cursor char is not
    // located in the buffer cursor line, which implies code logic error.
    let cursor = state.prompt_buffer.cursor();
    let (cursor_line, cursor_char): (usize, usize) = line_wraps[cursor.line]
        .iter()
        .try_fold(
            (
                // Number of visual lines before the buffer cursor line.
                line_wraps
                    .iter()
                    .take(cursor.line)
                    .map(|wrap| wrap.len())
                    .sum(),
                // Number of actual bytes before the buffer cursor char.
                0,
            ),
            |(cursor_line, num_bytes), &wrap| {
                let wrap_end = num_bytes + wrap.len();

                if cursor.char > wrap_end {
                    // If the actual cursor char is beyond the current wrap, goto next wrap.
                    Ok((cursor_line + 1, wrap_end))
                } else {
                    // If the actual cursor char is in current wrap, compute visual cursor char
                    // in this wrap (the visual length of the preifx string) and return it.
                    let cursor_char = wrap[..(cursor.char - num_bytes)].width();

                    Err((cursor_line, cursor_char))
                }
            },
        )
        .unwrap_err();

    let lines: Vec<_> = line_wraps.into_iter().flatten().collect();

    if let Some((scroll, offset, height)) = context.offset(lines.len()).params() {
        let area = Rect::new(area.x, area.y + offset, area.width, height);
        let [prompt_area, paragraph_area] =
            Layout::horizontal([Constraint::Length(1), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);

        Prompt::new(Style::default().fg(Color::Green))
            .scroll(scroll)
            .render(prompt_area, frame.buffer_mut());

        Paragraph::new(lines)
            .scroll(scroll)
            .render(paragraph_area, frame.buffer_mut());

        if matches!(state.mode, Mode::Prompt) && let Some(y_offset) = (cursor_line as u16).checked_sub(scroll)
            && y_offset < area.height
            && let x_offset = cursor_char as u16 + 2 // 2 for prefix width
            && x_offset < area.width
        {
            frame.set_cursor_position((area.x + x_offset, area.y + y_offset))
        }
    }
}

fn render_prompt_paragraph(
    content: &str,
    color: Color,
    context: &mut RangeContext,
    area: Rect,
    buf: &mut Buffer,
) {
    let lines = wrap_content(content, area.width as usize - 2); // Leave 2 for prompt width.

    if let Some((scroll, offset, height)) = context.offset(lines.len()).params() {
        let area = Rect::new(area.x, area.y + offset, area.width, height);
        let [prompt_area, paragraph_area] =
            Layout::horizontal([Constraint::Length(1), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);

        Prompt::new(Style::default().fg(color))
            .scroll(scroll)
            .render(prompt_area, buf);

        Paragraph::new(lines)
            .scroll(scroll)
            .render(paragraph_area, buf);
    }
}

fn render_paragraph(content: &str, context: &mut RangeContext, area: Rect, buf: &mut Buffer) {
    let lines = wrap_content(content, area.width as usize);

    if let Some((scroll, offset, height)) = context.offset(lines.len()).params() {
        let area = Rect::new(area.x, area.y + offset, area.width, height);
        Paragraph::new(lines).scroll(scroll).render(area, buf);
    }
}

fn render_separator(context: &mut RangeContext, area: Rect, buf: &mut Buffer) {
    if let Some((scroll, offset, height)) = context.offset(1).params() {
        let area = Rect::new(area.x, area.y + offset, area.width, height);
        Separator::new(" ").scroll(scroll).render(area, buf);
    }
}

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

fn wrap_content(content: &str, width: usize) -> Vec<&str> {
    content
        .lines()
        .flat_map(|line| wrap_line(line, width))
        .collect()
}
