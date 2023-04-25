pub use crate::text;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Layouter {
    byte_index: usize,
    position: Position,
}

impl Layouter {
    fn layout_line(&mut self, line: &str, handle_event: &mut impl FnMut(Event<'_>)) {
        self.layout_virtual_line(line, handle_event);
    }

    fn layout_virtual_line(&mut self, virtual_line: &str, handle_event: &mut impl FnMut(Event<'_>)) {
        use crate::str::StrExt;

        self.dispatch_event(EventKind::VirtualLineStart, virtual_line, handle_event);
        for grapheme in virtual_line.graphemes() {
            self.layout_grapheme(grapheme, handle_event);
        }
        self.dispatch_event(EventKind::VirtualLineEnd, virtual_line, handle_event);
        self.position.row_index += 1;
        self.position.column_index = 0;
    }

    fn layout_grapheme(&mut self, grapheme: &str, handle_event: &mut impl FnMut(Event<'_>)) {
        use crate::char::CharExt;

        self.dispatch_event(EventKind::GraphemeStart, grapheme, handle_event);
        self.byte_index += grapheme.len();
        self.position.column_index += grapheme
            .chars()
            .map(|char| char.column_count())
            .sum::<usize>();
        self.dispatch_event(EventKind::GraphemeEnd, grapheme, handle_event);
    }

    fn dispatch_event(&self, kind: EventKind, string: &str, handle_event: &mut impl FnMut(Event<'_>)) {
        handle_event(Event {
            position: self.position,
            byte_index: self.byte_index,
            string: string,
            kind,
        });
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Event<'a> {
    pub kind: EventKind,
    pub byte_index: usize,
    pub position: Position,
    pub string: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EventKind {
    VirtualLineStart,
    VirtualLineEnd,
    GraphemeStart,
    GraphemeEnd,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub row_index: usize,
    pub column_index: usize,
}

pub fn layout_line(line: &str, mut handle_event: impl FnMut(Event<'_>)) {
    Layouter {
        byte_index: 0,
        position: Position::default(),
    }.layout_line(line, &mut handle_event)
}