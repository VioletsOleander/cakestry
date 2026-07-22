use crate::widget::{HeightMeasurable, Request, Response, Scrollable, TextArea};
use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Widget, Wrap};
use std::cmp;

#[derive(Default)]
pub struct Session {
    messsages: Vec<Message>,
    textarea: TextArea,
    /// The view's start index in y coordinate in the document
    view_start: usize,
    /// The view's end index in y coordinate in the document
    view_end: usize,
    /// Offset in y coordinate in the document
    offset: usize,
}

impl Session {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        // The render area in y coordinate in the document is [view_start, view_end).
        self.offset = 0;
        self.view_end = self.view_start + area.height as usize;

        for message in &self.messsages {
            if self.offset >= self.view_end {
                return;
            }

            let paragraph = Paragraph::new(message.content.as_str()).wrap(Wrap { trim: true });

            if message.from_user {
                let request = Request::new(paragraph);
                self.render_widget(request, area, buf);
            } else {
                let response = Response::new(paragraph);
                self.render_widget(response, area, buf);
            }
        }

        // let textarea_start = document_y;
        //
        // // TextArea lies below view
        // if textarea_start >= view_end {
        //     return;
        // }
        //
        // let y_offset = textarea_start - view_start;
        // let visible_height = view_end - textarea_start;
        //
        // let textarea_area = Rect::new(
        //     area.x,
        //     area.y + y_offset as u16,
        //     area.width,
        //     visible_height as u16,
        // );
        //
        self.textarea.render(textarea_area, area, buf);
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Enter => {
                    if self.textarea.is_empty() {
                        return;
                    }

                    self.messsages
                        .push(Message::new(self.textarea.lines().join("\n"), true));
                    self.textarea.clear();
                }
                _ => {
                    self.textarea.input(key);
                }
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollDown => {
                    self.view_start = self.view_start.saturating_add(1);
                }
                MouseEventKind::ScrollUp => {
                    self.view_start = self.view_start.saturating_sub(1);
                }
                _ => (),
            },
            _ => (),
        }
    }

    /// Render given widget into the visible view in the given buffer.
    fn render_widget(
        &mut self,
        widget: impl Widget + Scrollable + HeightMeasurable,
        session_area: Rect,
        session_buf: &mut Buffer,
    ) {
        let widget_height = widget.height(session_area.width);
        let widget_start = self.offset;
        let widget_end = widget_start + widget_height;

        // Widget lies above the view
        if widget_end <= self.view_start {
            self.offset += widget_height;
            return;
        }

        // Widget overlaps with view
        let visible_start = cmp::max(self.view_start, widget_start);
        let visible_end = cmp::min(self.view_end, widget_end);

        let offset = visible_start - self.view_start;
        let visible_height = visible_end - visible_start;

        let skip = (visible_start - widget_start) as u16;
        let area = Rect::new(
            session_area.x,
            session_area.y + offset as u16,
            session_area.width,
            visible_height as u16,
        );

        widget.scroll(skip).render(area, session_buf);
        self.offset += widget_height;
    }
}

#[derive(Default)]
struct Message {
    content: String,
    from_user: bool,
}

impl Message {
    pub fn new(content: String, from_user: bool) -> Self {
        Message {
            content: content,
            from_user: from_user,
        }
    }
}
