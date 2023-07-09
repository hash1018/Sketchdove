use std::{cell::RefCell, rc::Rc};

use lib::figure::Figure;

#[derive(Default)]
pub struct FigureList {
    list: Rc<RefCell<Vec<Box<dyn Figure>>>>,
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
        }
    }

    pub fn push(&self, figure: Box<dyn Figure>) {
        self.list.borrow_mut().push(figure);
    }

    pub fn append(&self, mut figures: Vec<Box<dyn Figure>>) {
        self.list.borrow_mut().append(&mut figures);
    }

    pub fn list(&self) -> Rc<RefCell<Vec<Box<dyn Figure>>>> {
        self.list.clone()
    }
}

#[derive(Default)]
pub struct SharedUsers {
    list: Rc<RefCell<Vec<SharedUser>>>,
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
        }
    }

    pub fn push(&self, user: SharedUser) {
        self.list.borrow_mut().push(user);
    }

    pub fn append(&self, mut users: Vec<SharedUser>) {
        self.list.borrow_mut().append(&mut users);
    }

    pub fn remove(&self, user_id: String) {
        let position = self
            .list
            .borrow()
            .iter()
            .position(|user| user.user_id == user_id);
        if let Some(position) = position {
            self.list.borrow_mut().remove(position);
        }
    }

    pub fn list(&self) -> Rc<RefCell<Vec<SharedUser>>> {
        self.list.clone()
    }
}

#[derive(Default, Debug)]
pub struct SharedUser {
    user_id: String,
    #[allow(dead_code)]
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
