use crate::session::Session;
use crate::widget::{HeightMeasurable, Query, Renderable, Reply, Scrollable};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use std::cmp;

impl Session {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        // The render area in y coordinate in the document is [view_start, view_end).
        self.offset = 0;
        self.view_end = self.view_start + area.height as usize;

        // for message in &self.messsages {
        //     if self.offset >= self.view_end {
        //         return;
        //     }
        //
        //     if message.from_user() {
        //         let mut request = Request::new(message.content());
        //         self.offset += self.render_widget(&mut request, area, buf);
        //     } else {
        //         let mut response = Response::new(message.content());
        //         self.offset += self.render_widget(&mut response, area, buf);
        //     }
        // }

        if self.offset >= self.view_end {
            return;
        }

        let mut input_query = Query::new(self.user_input.lines());
        self.render_widget(&mut input_query, area, buf);
    }

    /// Render given widget into the visible view in the given buffer, and return the rendered widget's height.
    fn render_widget(
        &self,
        widget: &mut (impl Renderable + Scrollable + HeightMeasurable),
        session_area: Rect,
        session_buf: &mut Buffer,
    ) -> usize {
        let widget_height = widget.height(session_area.width);
        let widget_start = self.offset;
        let widget_end = widget_start + widget_height;

        // Widget lies above the view
        if widget_end <= self.view_start {
            return widget_height;
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

        widget.scroll(skip);
        widget.render(area, session_buf);
        widget_height
    }
}
