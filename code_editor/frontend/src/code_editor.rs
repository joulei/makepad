use {
    makepad_code_editor_core::{cursor_set, layout, text, text::Text, Cursor, CursorSet, Session},
    makepad_widgets::*,
    std::iter::Peekable,
};

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::theme::*;

    DrawSelection = {{DrawSelection}} {
        uniform gloopiness: 8.0
        uniform border_radius: 2.0

        fn vertex(self) -> vec4 { // custom vertex shader because we widen the draweable area a bit for the gloopiness
            let clipped: vec2 = clamp(
                self.geom_pos * vec2(self.rect_size.x + 16., self.rect_size.y) + self.rect_pos - vec2(8., 0.),
                self.draw_clip.xy,
                self.draw_clip.zw
            );
            self.pos = (clipped - self.rect_pos) / self.rect_size;
            return self.camera_projection * (self.camera_view * (
                self.view_transform * vec4(clipped.x, clipped.y, self.draw_depth + self.draw_zbias, 1.)
            ));
        }

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
            if self.prev_w > 0. {
                sdf.box(self.prev_x, -self.rect_size.y, self.prev_w, self.rect_size.y, self.border_radius);
                sdf.gloop(self.gloopiness);
            }
            if self.next_w > 0. {
                sdf.box(self.next_x, self.rect_size.y, self.next_w, self.rect_size.y, self.border_radius);
                sdf.gloop(self.gloopiness);
            }
            return sdf.fill(#08f8);
        }
    }

    CodeEditor = {{CodeEditor}} {
        draw_grapheme: {
            draw_depth: 0.0,
            text_style: <FONT_CODE> {}
        }
        draw_selection: {
            draw_depth: 1.0,
        }
        draw_caret: {
            draw_depth: 2.0,
            color: #f00
        }
    }
}

#[derive(Live, LiveHook)]
pub struct CodeEditor {
    draw_grapheme: DrawText,
    draw_selection: DrawSelection,
    draw_caret: DrawColor,
}

impl CodeEditor {
    pub fn draw(&mut self, cx: &mut Cx2d, session: &Session) {
        let cell_size =
            self.draw_grapheme.text_style.font_size * self.draw_grapheme.get_monospace_base(cx);
        let mut drawer = Drawer::new(
            &mut self.draw_grapheme,
            &mut self.draw_selection,
            &mut self.draw_caret,
            cell_size,
            session.cursors(),
        );
        drawer.draw_text(cx, session.document().borrow().text());
    }

    pub fn handle_event(&mut self, _cx: &mut Cx, _event: &Event) {
        // TODO
    }
}

pub struct Drawer<'a> {
    draw_grapheme: &'a mut DrawText,
    draw_selection: &'a mut DrawSelection,
    draw_caret: &'a mut DrawColor,
    cell_size: DVec2,
    text_position: text::Position,
    layout_position: layout::Position,
    draw_position: DVec2,
    active_cursor: Option<ActiveCursor>,
    cursors: Peekable<cursor_set::Iter<'a>>,
    prev_selection: Option<Rect>,
    selection: Option<Rect>,
}

impl<'a> Drawer<'a> {
    fn new(
        draw_grapheme: &'a mut DrawText,
        draw_selection: &'a mut DrawSelection,
        draw_caret: &'a mut DrawColor,
        cell_size: DVec2,
        cursors: &'a CursorSet,
    ) -> Self {
        Self {
            draw_grapheme,
            draw_selection,
            draw_caret,
            cell_size,
            text_position: text::Position::default(),
            layout_position: layout::Position::default(),
            draw_position: DVec2::default(),
            active_cursor: None,
            cursors: cursors.iter().peekable(),
            prev_selection: None,
            selection: None,
        }
    }

    fn draw_text(&mut self, cx: &mut Cx2d, text: &Text) {
        for line in text.as_lines() {
            self.draw_line(cx, line);
            self.text_position.line_index += 1;
            self.layout_position.row_index += 1;
        }
    }

    fn draw_line(&mut self, cx: &mut Cx2d, line: &str) {
        use makepad_code_editor_core::layout::EventKind::*;

        self.check_cursor_ends_right_before(cx);
        let start_row_index = self.layout_position.row_index;
        layout::layout_line(line, |event| {
            self.text_position.byte_index = event.byte_index;
            self.layout_position = layout::Position {
                row_index: start_row_index + event.position.row_index,
                column_index: event.position.column_index,
            };
            self.draw_position = DVec2 {
                x: self.layout_position.column_index as f64 * self.cell_size.x,
                y: self.layout_position.row_index as f64 * self.cell_size.y,
            };
            match event.kind {
                VirtualLineEnd => {
                    if let Some(cursor) = self.active_cursor {
                        self.draw_cursor(cx, cursor);
                    }
                }
                GraphemeStart => {
                    self.check_cursor_ends_right_after(cx);
                    self.draw_grapheme(cx, event.string);
                    self.check_cursor_starts_right_before();
                }
                GraphemeEnd => {
                    self.check_cursor_ends_right_before(cx);
                    self.check_cursor_starts_right_after();
                }
                _ => {}
            }
        });
        self.check_cursor_starts_right_after();
    }

    fn draw_grapheme(&mut self, cx: &mut Cx2d, grapheme: &str) {
        self.draw_grapheme
            .draw_abs(cx, self.draw_position, grapheme);
    }

    fn check_cursor_starts_right_before(&mut self) {
        if self.cursors.peek().map_or(false, |cursor| {
            cursor.start().is_right_before(self.text_position)
        }) {
            self.handle_cursor_start();
        }
    }

    fn check_cursor_starts_right_after(&mut self) {
        if self.cursors.peek().map_or(false, |cursor| {
            cursor.start().is_right_before(self.text_position)
        }) {
            self.handle_cursor_start();
        }
    }

    fn check_cursor_ends_right_before(&mut self, cx: &mut Cx2d) {
        if self.active_cursor.as_ref().map_or(false, |active_cursor| {
            active_cursor
                .cursor
                .end()
                .is_right_after(self.text_position)
        }) {
            self.handle_cursor_end(cx);
        }
    }

    fn check_cursor_ends_right_after(&mut self, cx: &mut Cx2d) {
        if self.active_cursor.as_ref().map_or(false, |active_cursor| {
            active_cursor
                .cursor
                .end()
                .is_right_before(self.text_position)
        }) {
            self.handle_cursor_end(cx);
        }
    }

    fn handle_cursor_start(&mut self) {
        self.active_cursor = Some(ActiveCursor {
            cursor: self.cursors.next().unwrap(),
            start_x: self.draw_position.x,
        });
    }

    fn handle_cursor_end(&mut self, cx: &mut Cx2d) {
        let cursor = self.active_cursor.take().unwrap();
        self.draw_cursor(cx, cursor);
    }

    fn draw_cursor(&mut self, cx: &mut Cx2d, cursor: ActiveCursor) {
        self.draw_selection(
            cx,
            Some(
                if cursor.cursor.start().position.line_index == self.text_position.line_index {
                    cursor.start_x
                } else {
                    0.0
                },
            ),
        );
        if cursor.cursor.end().position == self.text_position {
            self.draw_selection(cx, None);
        }
        if cursor.cursor.caret.position.line_index == self.text_position.line_index {
            self.draw_caret(cx);
        }
    }

    fn draw_selection(&mut self, cx: &mut Cx2d, start_x: Option<f64>) {
        let next_selection = start_x.map(|start_x| Rect {
            pos: DVec2 {
                x: start_x,
                y: self.draw_position.y,
            },
            size: DVec2 {
                x: self.draw_position.x - start_x,
                y: self.cell_size.y,
            },
        });
        if let Some(selection) = self.selection {
            if let Some(prev_selection) = self.prev_selection {
                self.draw_selection.prev_x = prev_selection.pos.x as f32;
                self.draw_selection.prev_w = prev_selection.size.x as f32;
            } else {
                self.draw_selection.prev_x = 0.0;
                self.draw_selection.prev_w = 0.0;
            }
            if let Some(next_selection) = next_selection {
                self.draw_selection.next_x = next_selection.pos.x as f32;
                self.draw_selection.next_w = next_selection.size.x as f32;
            } else {
                self.draw_selection.next_x = 0.0;
                self.draw_selection.next_w = 0.0;
            }
            self.draw_selection.draw_abs(cx, selection);
        }
        self.prev_selection = self.selection;
        self.selection = next_selection;
    }

    fn draw_caret(&mut self, cx: &mut Cx2d) {
        self.draw_caret.draw_abs(
            cx,
            Rect {
                pos: self.draw_position,
                size: DVec2 {
                    x: 2.0,
                    y: self.cell_size.y,
                },
            },
        );
    }
}

#[derive(Clone, Copy)]
struct ActiveCursor {
    cursor: Cursor,
    start_x: f64,
}

#[derive(Live, LiveHook)]
#[repr(C)]
struct DrawSelection {
    draw_super: DrawQuad,
    prev_x: f32,
    prev_w: f32,
    next_x: f32,
    next_w: f32,
}
