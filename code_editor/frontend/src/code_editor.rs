use {makepad_code_editor_core::Session, makepad_widgets::*};

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
        use makepad_code_editor_core::{layout, layout::Event};

        let DVec2 {
            x: column_width,
            y: row_height,
        } = self.draw_grapheme.text_style.font_size * self.draw_grapheme.get_monospace_base(cx);
        layout::layout(
            session.document().borrow().text(),
            &session.cursors(),
            |layout_position, element| {
                let screen_position = DVec2 {
                    x: layout_position.column_index as f64 * column_width,
                    y: layout_position.row_index as f64 * row_height,
                };
                match element {
                    Event::Grapheme(grapheme) => {
                        self.draw_grapheme.draw_abs(cx, screen_position, grapheme);
                    }
                    Event::SelectionFragment(column_count) => {
                        self.draw_selection.draw(
                            cx,
                            Some(Rect {
                                pos: screen_position,
                                size: DVec2 {
                                    x: column_count as f64 * column_width,
                                    y: row_height,
                                },
                            }),
                        );
                    }
                    Event::SelectionEnd => {
                        self.draw_selection.draw(cx, None);
                    }
                    Event::Caret => {
                        self.draw_caret.draw_abs(
                            cx,
                            Rect {
                                pos: screen_position,
                                size: DVec2 {
                                    x: 2.0,
                                    y: row_height,
                                },
                            },
                        );
                    }
                }
            },
        );
    }

    pub fn handle_event(&mut self, _cx: &mut Cx, _event: &Event) {
        // TODO
    }
}

#[derive(Live, LiveHook)]
#[repr(C)]
pub struct DrawSelection {
    #[rust]
    prev_rect: Option<Rect>,
    #[rust]
    rect: Option<Rect>,
    draw_super: DrawQuad,
    prev_x: f32,
    prev_w: f32,
    next_x: f32,
    next_w: f32,
}

impl DrawSelection {
    fn draw(&mut self, cx: &mut Cx2d, next_rect: Option<Rect>) {
        if let Some(rect) = self.rect {
            if let Some(prev_rect) = self.prev_rect {
                self.prev_x = prev_rect.pos.x as f32;
                self.prev_w = prev_rect.size.x as f32;
            } else {
                self.prev_x = 0.0;
                self.prev_w = 0.0;
            }
            if let Some(next_rect) = next_rect {
                self.next_x = next_rect.pos.x as f32;
                self.next_w = next_rect.size.x as f32;
            } else {
                self.next_x = 0.0;
                self.next_w = 0.0;
            }
            self.draw_abs(cx, rect);
        }
        self.prev_rect = self.rect;
        self.rect = next_rect;
    }
}
