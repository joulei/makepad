use {
    crate::{rc::WeakPtrEq, Session},
    makepad_futures::executor::Spawner,
    std::{
        cell::RefCell,
        collections::HashSet,
        future::Future,
        rc::{Rc, Weak},
    },
};

#[derive(Debug)]
pub struct Document {
    sessions: HashSet<WeakPtrEq<RefCell<Session>>>,
    lines: Vec<String>,
}

impl Document {
    pub fn new(spawner: &Spawner, load: impl Future<Output = Vec<String>> + 'static) -> Rc<RefCell<Self>> {
        let document = Rc::new(RefCell::new(Self {
            sessions: HashSet::new(),
            lines: ["Loading...".into()].into(),
        }));
        spawner
            .spawn({
                let document = document.clone();
                async move {
                    let lines = load.await;
                    let mut document = document.borrow_mut();
                    document.lines = lines;
                }
            })
            .unwrap();
        document
    }

    pub fn sessions(&self) -> &HashSet<WeakPtrEq<RefCell<Session>>> {
        &self.sessions
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn insert_session(&mut self, session: Weak<RefCell<Session>>) {
        self.sessions.insert(WeakPtrEq(session));
    }

    pub fn remove_session(&mut self, session: Weak<RefCell<Session>>) {
        self.sessions.remove(&WeakPtrEq(session));
    }
}
