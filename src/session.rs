use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Paragraph, Widget, Wrap};
use ratatui_textarea::TextArea;
use std::cmp;

#[derive(Default)]
pub struct Session {
    messsages: Vec<Message>,
    /// The view's start index in y coordinate in the document
    /// (assume there is a larger document, containing a viewport rendered to the screen)
    view_start: usize,
    textarea: TextArea<'static>,
}

impl Session {
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // Render area in y coordinate in the document: [view_start, view_end)
        let view_start = self.view_start;
        let view_end = self.view_start + area.height as usize;

        let mut paragraph_start: usize = 0; // y in the document

        for message in &self.messsages {
            let paragraph = self.make_paragraph(message);
            let height = paragraph.line_count(area.width);

            let paragraph_end = paragraph_start + height;

            // Paragraph lies below view
            if paragraph_start >= view_end {
                return;
            }

            // Paragraph lies above view
            if paragraph_end <= view_start {
                paragraph_start += height;
                continue;
            }

            // Paragraph overlaps with view
            let visible_start = cmp::max(view_start, paragraph_start);
            let visible_end = cmp::min(view_end, paragraph_end);

            let skipped_height = visible_start - paragraph_start;
            let y_offset = visible_start - view_start;
            let visible_height = visible_end - visible_start;

            let paragraph_area = Rect::new(
                area.x,
                area.y + y_offset as u16,
                area.width,
                visible_height as u16,
            );

            paragraph
                .scroll((skipped_height as u16, 0))
                .render(paragraph_area, buf);

            paragraph_start += height;
        }

        let textarea_start = paragraph_start;
        let y_offset = textarea_start - view_start;
        let visible_height = view_end - textarea_start;

        let textarea_area = Rect::new(
            area.x,
            area.y + y_offset as u16,
            area.width,
            visible_height as u16,
        );

        self.textarea.render(textarea_area, buf);
    }

    fn make_paragraph<'a>(&self, message: &'a Message) -> Paragraph<'a> {
        let paragraph = Paragraph::new(message.content.as_str());

        if message.from_user {
            paragraph.wrap(Wrap { trim: true }).block(Block::bordered())
        } else {
            paragraph.wrap(Wrap { trim: true })
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Enter => {
                        if self.textarea.is_empty() {
                            return;
                        }

                        self.messsages
                            .push(Message::new(self.textarea.lines().join("\n"), true));
                        self.textarea.clear();
                    }
                    KeyCode::Char('j') if key.modifiers == KeyModifiers::CONTROL => {
                        // This also matches Ctrl-Enter
                        self.textarea.insert_newline();
                    }
                    _ => {
                        self.textarea.input(key);
                    }
                }
            }
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
