use {makepad_code_editor_core::Session, makepad_widgets::*};

live_design! {
    import makepad_widgets::theme::*;

    CodeEditor = {{CodeEditor}} {
        draw_text: {
            text_style: <FONT_CODE> {}
        }
    }
}

#[derive(Live, LiveHook)]
pub struct CodeEditor {
    draw_text: DrawText,
}

impl CodeEditor {
    pub fn draw(&mut self, cx: &mut Cx2d, session: &Session) {
        let DVec2 {
            x: column_width,
            y: row_height,
        } = self.draw_text.text_style.font_size * self.draw_text.get_monospace_base(cx);
        let mut row_index = 10;
        for line in session.document().borrow().text().as_lines() {
            self.draw_text.draw(
                cx,
                DVec2 {
                    x: 0.0,
                    y: row_index as f64 * row_height,
                },
                line,
            );
            row_index += 1;
        }
    }

    pub fn handle_event(&mut self, _cx: &mut Cx, _event: &Event) {
        // TODO
    }
}
