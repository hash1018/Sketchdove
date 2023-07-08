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
    UserLeft,
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
        let is_you = user.is_you;

        self.list.borrow_mut().push(user);

        if !is_you {
            *self.is_modified.borrow_mut() = true;
            *self.modified_reason.borrow_mut() = Some(SharedUsersModifiedReason::UserJoined);
        }
    }

    pub fn append(&self, mut users: Vec<SharedUser>) {
        self.list.borrow_mut().append(&mut users);
        *self.is_modified.borrow_mut() = true;
        *self.modified_reason.borrow_mut() = Some(SharedUsersModifiedReason::UserJoined);
    }

    pub fn remove(&self, user_id: String) {
        let position = self
            .list
            .borrow()
            .iter()
            .position(|user| user.user_id == user_id);
        if let Some(position) = position {
            self.list.borrow_mut().remove(position);
            *self.is_modified.borrow_mut() = true;
            *self.modified_reason.borrow_mut() = Some(SharedUsersModifiedReason::UserLeft);

            let str = format!("user left! current users {0:?}", *self.list.borrow());

            log::info!("{str}");
        }
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

    pub fn reset_modified_reason(&self) {
        *self.modified_reason.borrow_mut() = None;
    }
}

#[derive(Default, Debug)]
pub struct SharedUser {
    user_id: String,
    is_you: bool,
}

impl SharedUser {
    pub fn new(user_id: String, is_you: bool) -> Self {
        Self { user_id, is_you }
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}
