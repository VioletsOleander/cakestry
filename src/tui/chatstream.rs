use std::cmp;

use ratatui::Frame;
use ratatui::layout::{Offset, Rect};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::state::State;

mod buffer;
mod prompt;
mod response;
mod spacer;

use prompt::PromptView;
use response::ResponseView;
use spacer::Spacer;

pub struct ChatStream;

/// Range in absolute y coordinates.
struct Range {
    start: usize,
    end: usize,
}

impl ChatStream {
    // The chatstream is allowed to have a height larger than area.height. The rendered viewport
    // is determined by the scroll offset and a part of the stream will be rendered to it.
    pub fn render(state: &State, area: Rect, frame: &mut Frame) {
        let viewport = Range::new(
            state.stream_scroll,
            state.stream_scroll + area.height as usize,
        );
        let mut widget = Range::new(0, 0);

        for exchange in &state.exchanges {
            if let Ok(prompt) = PromptView::build(&exchange.prompt, area.width as usize)
                && let Some(visible) =
                    get_visible_range(&viewport, widget.proceed().extend(prompt.height()))
            {
                let scroll = (visible.start - widget.start) as u16;
                let area = area.offset(Offset::new(0, (visible.start - viewport.start) as i32));

                prompt.scroll(scroll).render(area, frame);
            }

            if let Some(visible) = get_visible_range(&viewport, widget.proceed().extend(1)) {
                let area = area.offset(Offset::new(0, (visible.start - viewport.start) as i32));
                Spacer::render(area, frame);
            }

            if let Ok(response) = ResponseView::build(&exchange.response, area.width as usize)
                && let Some(visible) =
                    get_visible_range(&viewport, widget.proceed().extend(response.height()))
            {
                let scroll = (visible.start - widget.start) as u16;
                let area = area.offset(Offset::new(0, (visible.start - viewport.start) as i32));

                response.scroll(scroll).render(area, frame);
            }

            if let Some(visible) = get_visible_range(&viewport, widget.proceed().extend(1)) {
                let area = area.offset(Offset::new(0, (visible.start - viewport.start) as i32));
                Spacer::render(area, frame);
            }
        }
    }
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Extend range end by `delta`.
    fn extend(&mut self, delta: usize) -> &mut Self {
        self.end += delta;
        self
    }

    /// Set range start to range end.
    fn proceed(&mut self) -> &mut Self {
        self.start = self.end;
        self
    }
}

/// Compute and return the widget's visible range in viewport.
///
/// Return `None` if the widget is outside the viewport.
fn get_visible_range(viewport: &Range, widget: &Range) -> Option<Range> {
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

/// Wrap `line` into possibly multiple lines according to `width`.
///
/// `width` is supposed to be Unicode width, and is required to be positive.
fn wrap_line(line: &str, width: usize) -> Vec<&str> {
    let mut lines = Vec::new();

    let mut line_width = 0;
    let mut start_idx = 0;

    for (idx, grapheme) in line.grapheme_indices(true) {
        let grapheme_width = grapheme.width();

        if line_width + grapheme_width > width {
            // Last line ranges from the start grapheme to last grapheme.
            lines.push(&line[start_idx..idx]);

            // This line starts from current grapheme.
            start_idx = idx;
            line_width = grapheme_width;
        } else {
            line_width += grapheme_width;
        }
    }

    // Trailing graphemes
    lines.push(&line[start_idx..]);

    lines
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_wrap_ascii() {
        let line = "Hello World";
        let width = 11;

        let lines = wrap_line(line, width);

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "Hello World");
    }

    #[test]
    fn one_wrap_ascii() {
        let line = "Hello World";
        let width = 10;

        let lines = wrap_line(line, width);

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "Hello Worl");
        assert_eq!(lines[1], "d");
    }

    #[test]
    fn two_wrap_ascii() {
        let line = "Hello World";
        let width = 5;

        let lines = wrap_line(line, width);

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "Hello");
        assert_eq!(lines[1], " Worl");
        assert_eq!(lines[2], "d");
    }

    #[test]
    fn no_wrap_cjk() {
        let line = "你好 世界";
        let width = 9;

        let lines = wrap_line(line, width);

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "你好 世界");
    }

    #[test]
    fn one_wrap_cjk() {
        let line = "你好 世界";
        let width = 8;

        let lines = wrap_line(line, width);

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "你好 世");
        assert_eq!(lines[1], "界");
    }

    #[test]
    fn two_wrap_cjk() {
        let line = "你好 世界";
        let width = 4;

        let lines = wrap_line(line, width);

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "你好");
        assert_eq!(lines[1], " 世");
        assert_eq!(lines[2], "界");
    }
}
