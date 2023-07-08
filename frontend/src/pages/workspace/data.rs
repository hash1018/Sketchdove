use std::{cell::RefCell, rc::Rc};

use lib::figure::Figure;

#[derive(Default)]
pub struct FigureList {
    list: Rc<RefCell<Vec<Box<dyn Figure>>>>,
    is_modified: RefCell<bool>,
}

impl PartialEq for FigureList {
    fn eq(&self, other: &Self) -> bool {
        self.list.borrow().len() == other.list.borrow().len()
    }
}

impl FigureList {
    pub fn new() -> FigureList {
        FigureList {
            list: Rc::new(RefCell::new(Vec::new())),
            is_modified: RefCell::new(false),
        }
    }

    pub fn push(&self, figure: Box<dyn Figure>) {
        self.list.borrow_mut().push(figure);
        *self.is_modified.borrow_mut() = true;
    }

    pub fn append(&self, mut figures: Vec<Box<dyn Figure>>) {
        self.list.borrow_mut().append(&mut figures);
        *self.is_modified.borrow_mut() = true;
    }

    pub fn list(&self) -> Rc<RefCell<Vec<Box<dyn Figure>>>> {
        self.list.clone()
    }

    pub fn is_modified(&self) -> bool {
        *self.is_modified.borrow()
    }

    pub fn reset_modified(&self) {
        *self.is_modified.borrow_mut() = false;
    }
}

#[derive(Clone)]
pub enum SharedUsersModifiedReason {
    UserJoined,
}

#[derive(Default)]
pub struct SharedUsers {
    list: Rc<RefCell<Vec<SharedUser>>>,
    is_modified: RefCell<bool>,
    modified_reason: RefCell<Option<SharedUsersModifiedReason>>,
}

impl PartialEq for SharedUsers {
    fn eq(&self, other: &Self) -> bool {
        self.list.borrow().len() == other.list.borrow().len()
    }
}

impl SharedUsers {
    pub fn new() -> Self {
        Self {
            list: Rc::new(RefCell::new(Vec::new())),
            is_modified: RefCell::new(false),
            modified_reason: RefCell::new(None),
        }
    }

    pub fn push(&self, user: SharedUser) {
        self.list.borrow_mut().push(user);
        *self.is_modified.borrow_mut() = true;
    }

    pub fn list(&self) -> Rc<RefCell<Vec<SharedUser>>> {
        self.list.clone()
    }

    pub fn is_modified(&self) -> bool {
        *self.is_modified.borrow()
    }

    pub fn modified_reason(&self) -> Option<SharedUsersModifiedReason> {
        self.modified_reason.borrow().clone()
    }

    pub fn reset_modified(&self) {
        *self.is_modified.borrow_mut() = false;
    }
}

#[derive(Default)]
pub struct SharedUser {
    user_id: String,
}

impl SharedUser {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}
