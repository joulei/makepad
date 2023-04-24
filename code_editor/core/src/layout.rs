pub use {
    crate::{
        cursor::{cursor_set, Cursor, CursorSet},
        {
            text,
            text::{Range, Text},
        },
    },
    std::iter::Peekable,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Position {
    pub row_index: usize,
    pub column_index: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Event<'a> {
    Grapheme(&'a str),
    SelectionFragment(usize),
    SelectionEnd,
    Caret,
}

struct Layouter<'a, F> {
    text_position: text::Position,
    position: Position,
    active_cursor: Option<ActiveCursor>,
    cursors: Peekable<cursor_set::Iter<'a>>,
    handle_event: F,
}

impl<'a, F> Layouter<'a, F> {
    fn new(cursors: &'a CursorSet, f: F) -> Self {
        Self {
            text_position: text::Position::default(),
            position: Position::default(),
            active_cursor: None,
            cursors: cursors.iter().peekable(),
            handle_event: f,
        }
    }
}

impl<'a, F> Layouter<'a, F>
where
    F: FnMut(Position, Event<'a>),
{
    fn layout(&mut self, text: &'a Text) {
        for line in text.as_lines().iter() {
            self.layout_line(line);
        }
    }

    fn layout_line(&mut self, line: &'a str) {
        self.handle_line_start();
        self.layout_virtual_line(line);
        self.handle_line_end();
        self.text_position.line_index += 1;
        self.text_position.byte_index = 0;
    }

    fn layout_virtual_line(&mut self, virtual_line: &'a str) {
        use crate::str::StrExt;

        self.handle_virtual_line_start();
        for grapheme in virtual_line.graphemes() {
            self.handle_grapheme_start();
            self.layout_grapheme(grapheme);
            self.handle_grapheme_end();
        }
        self.handle_virtual_line_end();
        self.position.row_index += 1;
        self.position.column_index = 0;
    }

    fn layout_grapheme(&mut self, grapheme: &'a str) {
        use crate::str::StrExt;

        self.dispatch_event(self.position, Event::Grapheme(grapheme));
        self.text_position.byte_index += grapheme.len();
        self.position.column_index += grapheme.column_count();
    }

    fn handle_line_start(&mut self) {
        if let Some(cursor) = &self.active_cursor {
            if cursor.cursor.end().is_right_before(self.text_position) {
                self.handle_cursor_end();
            }
        }
    }

    fn handle_line_end(&mut self) {
        if let Some(&cursor) = self.cursors.peek() {
            if cursor.start().is_right_after(self.text_position) {
                self.handle_cursor_start();
            }
        }
    }

    fn handle_virtual_line_start(&mut self) {}

    fn handle_virtual_line_end(&mut self) {
        if let Some(&active_cursor) = self.active_cursor.as_ref() {
            self.layout_cursor(active_cursor);
        }
    }

    fn handle_grapheme_start(&mut self) {
        if let Some(active_cursor) = &self.active_cursor {
            if active_cursor
                .cursor
                .end()
                .is_right_after(self.text_position)
            {
                self.handle_cursor_end();
            }
        }
        if let Some(&cursor) = self.cursors.peek() {
            if cursor.start().is_right_before(self.text_position) {
                self.handle_cursor_start();
            }
        }
    }

    fn handle_grapheme_end(&mut self) {
        if let Some(active_cursor) = &self.active_cursor {
            if active_cursor
                .cursor
                .end()
                .is_right_before(self.text_position)
            {
                self.handle_cursor_end();
            }
        }
        if let Some(&cursor) = self.cursors.peek() {
            if cursor.start().is_right_after(self.text_position) {
                self.handle_cursor_start();
            }
        }
    }

    fn handle_cursor_start(&mut self) {
        self.active_cursor = Some(ActiveCursor {
            cursor: self.cursors.next().unwrap(),
            start_column_index: self.position.column_index,
        });
    }

    fn handle_cursor_end(&mut self) {
        let active_cursor = self.active_cursor.take().unwrap();
        self.layout_cursor(active_cursor);
    }

    fn layout_cursor(&mut self, active_cursor: ActiveCursor) {
        let start_column_index =
            if active_cursor.cursor.start().position.line_index == self.text_position.line_index {
                active_cursor.start_column_index
            } else {
                0
            };
        self.dispatch_event(
            Position {
                row_index: self.position.row_index,
                column_index: start_column_index,
            },
            Event::SelectionFragment(if self.position.column_index == 0 {
                1
            } else {
                self.position.column_index - start_column_index
            }),
        );
        if active_cursor.cursor.end().position == self.text_position {
            self.dispatch_event(self.position, Event::SelectionEnd);
        }
        if active_cursor.cursor.caret.position.line_index == self.text_position.line_index {
            self.dispatch_event(self.position, Event::Caret)
        }
    }

    fn dispatch_event(&mut self, position: Position, event: Event<'a>) {
        (self.handle_event)(position, event);
    }
}

#[derive(Clone, Copy, Debug)]
struct ActiveCursor {
    cursor: Cursor,
    start_column_index: usize,
}

pub fn layout<'a>(
    text: &'a Text,
    cursors: &'a CursorSet,
    dispatch_event: impl FnMut(Position, Event<'a>),
) {
    let mut layouter = Layouter::new(cursors, dispatch_event);
    layouter.layout(text);
}
