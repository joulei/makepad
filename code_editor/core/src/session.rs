use {
    crate::{Cursor, cursor::Context, CursorSet, Document},
    std::{cell::RefCell, rc::Rc},
};

#[derive(Debug)]
pub struct Session {
    cursors: CursorSet,
    document: Rc<RefCell<Document>>,
}

impl Session {
    pub fn new(document: Rc<RefCell<Document>>) -> Rc<RefCell<Self>> {
        use crate::{
            cursor,
            cursor::{Affinity},
            text,
        };

        let session = Rc::new(RefCell::new(Self {
            cursors: [Cursor {
                caret: cursor::Position {
                    position: text::Position {
                        line_index: 12,
                        byte_index: 20,
                    },
                    affinity: Affinity::default(),
                },
                anchor: cursor::Position::default(),
            }]
            .into(),
            document: document.clone(),
        }));
        document
            .borrow_mut()
            .insert_session(Rc::downgrade(&session));
        session
    }

    pub fn document(&self) -> &Rc<RefCell<Document>> {
        &self.document
    }

    pub fn cursors(&self) -> &CursorSet {
        &self.cursors
    }

    pub fn update_cursors(&mut self, mut f: impl FnMut(Cursor, &Context) -> Cursor) {
        let document = self.document.borrow();
        let context = Context {
            lines: document.text().as_lines(),
        };
        self.cursors.update(|cursor| {
            f(cursor, &context)
        });
    }
}
