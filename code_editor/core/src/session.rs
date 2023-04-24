use {
    crate::{Document, SelectionSet},
    std::{cell::RefCell, rc::Rc},
};

#[derive(Debug)]
pub struct Session {
    selections: SelectionSet,
    document: Rc<RefCell<Document>>,
}

impl Session {
    pub fn new(document: Rc<RefCell<Document>>) -> Rc<RefCell<Self>> {
        use crate::{text::Position, Selection};

        let session = Rc::new(RefCell::new(Self {
            selections: [Selection {
                cursor: Position {
                    line_index: 12,
                    byte_index: 0,
                },
                anchor: Position::default(),
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

    pub fn selections(&self) -> &SelectionSet {
        &self.selections
    }
}
