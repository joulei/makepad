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
            return sdf.fill(#f008);
        }
    }

    CodeEditor = {{CodeEditor}} {
        draw_text: {
            text_style: <FONT_CODE> {}
        }
    }
}

#[derive(Live, LiveHook)]
pub struct CodeEditor {
    draw_text: DrawText,
    draw_selection: DrawSelection,
}

impl CodeEditor {
    pub fn draw(&mut self, cx: &mut Cx2d, session: &Session) {
        use {makepad_code_editor_core::{char::CharExt, str::StrExt, Position, Selection}, std::iter};

        struct ActiveSelection {
            selection: Selection,
            start_x: f64,
        }

        // This is a wrapper function around self.draw_selection that delays drawing by
        // one line, and keeps track of the last two rectangles we've drawn. In other
        // words, whenever we "draw" a rectangle, only the previous rectangle gets
        // drawn, using the rectangle before it (prev_rect) and after it (next_rect)
        // to create a gloopy rectangle.git
        let mut draw_selection = {
            let draw_selection = &mut self.draw_selection;
            let mut prev_rect: Option<Rect> = None;
            let mut rect: Option<Rect> = None;
            move |cx: &mut Cx2d, next_rect: Option<Rect>| {
                draw_selection.draw_depth = 2.0;
                if let Some(rect) = rect {
                    if let Some(prev_rect) = prev_rect {
                        draw_selection.prev_x = prev_rect.pos.x as f32;
                        draw_selection.prev_w = prev_rect.size.x as f32;
                    } else {
                        draw_selection.prev_x = 0.0;
                        draw_selection.prev_w = 0.0;
                    }
                    if let Some(next_rect) = next_rect {
                        draw_selection.next_x = next_rect.pos.x as f32;
                        draw_selection.next_w = next_rect.size.x as f32;
                    } else {
                        draw_selection.next_x = 0.0;
                        draw_selection.next_w = 0.0;
                    }
                    draw_selection.draw_abs(cx, rect);
                }
                prev_rect = rect;
                rect = next_rect;
            }
        };

        let DVec2 {
            x: column_width,
            y: row_height,
        } = self.draw_text.text_style.font_size * self.draw_text.get_monospace_base(cx);
        let mut selections = session.selections().iter().peekable();
        let mut active_selection_option: Option<ActiveSelection> = None;
        let mut row_index = 0;
        for (line_index, line) in session.document().borrow().text().as_lines().iter().enumerate() {
            let y = row_index as f64 * row_height;
            let mut column_index = 0;
            for (byte_index, grapheme) in line.grapheme_indices().chain(iter::once((line.len(), ""))) {
                let position = Position {
                    line_index,
                    byte_index,
                };
                let x = column_index as f64 * column_width;

                // Check if we have an active selection that ends at the current position or we've
                // reached the end of the current line while an active selection is present. If so,
                // draw a rectangle for the active selection on the current line.
                if let Some(active_selection) = &mut active_selection_option {
                    if active_selection.selection.end() == position || byte_index == line.len() {
                        // If we are drawing the first line of the active selection, its start
                        // x-coordinate is the coordinate we recorded when the selection became
                        // active. Otherwise, it's start x-coordinate is the start of the line.
                        let start_x = if active_selection.selection.start().line_index == line_index
                        {
                            active_selection.start_x
                        } else {
                            0.0
                        };
                        // Actually draw the rectangle for the active selection on the current line.
                        draw_selection(cx, Some(Rect {
                            pos: DVec2 { x: start_x, y },
                            size: DVec2 {
                                x: x - start_x,
                                y: row_height,
                            },
                        }));
                        // If the active selection ends at the current position, make sure we no
                        // longer have an active selection after this point.
                        if active_selection.selection.end() == position {
                            draw_selection(cx, None);
                            active_selection_option = None;
                        }
                    }
                }

                // Check if we have a selection that starts at the current position. If so, make it
                // the active selection.
                if let Some(&selection) = selections.peek() {
                    if selection.start() == position {
                        // Record the selection and its start x-coordinate. We need this later when
                        // drawing the first line of the active selection.
                        active_selection_option = Some(ActiveSelection {
                            selection,
                            start_x: x,
                        });
                        selections.next().unwrap();
                    }
                }
                self.draw_text.draw(cx, DVec2 { x, y }, grapheme);
                column_index += grapheme.chars().map(|char| char.column_count()).sum::<usize>();
            }
            row_index += 1;
        }
    }

    pub fn handle_event(&mut self, _cx: &mut Cx, _event: &Event) {
        // TODO
    }
}

#[derive(Live, LiveHook)]
#[repr(C)]
pub struct DrawSelection {
    draw_super: DrawQuad,
    prev_x: f32,
    prev_w: f32,
    next_x: f32,
    next_w: f32
}