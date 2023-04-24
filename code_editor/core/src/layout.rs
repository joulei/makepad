pub use {
    crate::{
        selection::Selection,
        selection_set,
        selection_set::SelectionSet,
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
pub enum Element<'a> {
    Grapheme(&'a str),
    Selection(usize),
}

struct Layouter<'a, F> {
    text_position: text::Position,
    position: Position,
    active_selection: Option<ActiveSelection>,
    selections: Peekable<selection_set::Iter<'a>>,
    f: F,
}

impl<'a, F> Layouter<'a, F> {
    fn new(selections: &'a SelectionSet, f: F) -> Self {
        Self {
            text_position: text::Position::default(),
            position: Position::default(),
            active_selection: None,
            selections: selections.iter().peekable(),
            f,
        }
    }
}

impl<'a, F> Layouter<'a, F>
where
    F: FnMut(Position, Element<'a>),
{
    fn layout(&mut self, text: &'a Text) {
        for line in text.as_lines().iter() {
            self.layout_line(line);
        }
    }

    fn layout_line(&mut self, line: &'a str) {
        use crate::str::StrExt;

        for grapheme in line.graphemes() {
            self.handle_grapheme_boundary();
            self.layout_grapheme(grapheme);
        }
        self.handle_grapheme_boundary();
        if let Some(&selection) = self.active_selection.as_ref() {
            self.layout_selection(selection);
        }
        self.text_position.line_index += 1;
        self.text_position.byte_index = 0;
        self.position.row_index += 1;
        self.position.column_index = 0;
    }

    fn layout_grapheme(&mut self, grapheme: &'a str) {
        use crate::char::CharExt;

        (self.f)(self.position, Element::Grapheme(grapheme));
        self.text_position.byte_index += grapheme.len();
        self.position.column_index += grapheme
            .chars()
            .map(|char| char.column_count())
            .sum::<usize>();
    }

    fn handle_grapheme_boundary(&mut self) {
        if let Some(selection) = &self.active_selection {
            if selection.end == self.text_position {
                let selection = self.active_selection.take().unwrap();
                self.layout_selection(selection);
            }
        }
        if let Some(&selection) = self.selections.peek() {
            if selection.start() == self.text_position {
                let selection = self.selections.next().unwrap();
                self.active_selection = Some(ActiveSelection {
                    start_line_index: selection.start().line_index,
                    end: selection.end(),
                    start_column_index: self.position.column_index,
                });
            }
        }
    }

    fn layout_selection(&mut self, selection: ActiveSelection) {
        let start_column_index = if selection.start_line_index == self.text_position.line_index {
            selection.start_column_index
        } else {
            0
        };
        (self.f)(
            Position {
                row_index: self.position.row_index,
                column_index: start_column_index,
            },
            Element::Selection(self.position.column_index - start_column_index),
        );
    }
}

#[derive(Clone, Copy, Debug)]
struct ActiveSelection {
    start_line_index: usize,
    end: text::Position,
    start_column_index: usize,
}

pub fn layout<'a>(
    text: &'a Text,
    selections: &'a SelectionSet,
    f: impl FnMut(Position, Element<'a>),
) {
    let mut layouter = Layouter::new(selections, f);
    layouter.layout(text);
}
