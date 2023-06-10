pub mod api;
pub mod client;
pub mod components;
pub mod pages;

use std::cell::RefCell;

use lib::user::User;

#[derive(PartialEq, Default)]
pub struct LoginedUser {
    inner: RefCell<Option<User>>,
}

impl LoginedUser {
    pub fn new() -> Self {
        LoginedUser {
            inner: RefCell::new(None),
        }
    }

    pub fn is_logined(&self) -> bool {
        self.inner.borrow().is_some()
    }

    pub fn login(&self, user: User) {
        *self.inner.borrow_mut() = Some(user);
    }

    pub fn logout(&self) {
        let _ = self.inner.borrow_mut().take();
    }

    pub fn user(&self) -> Option<User> {
        self.inner.borrow().clone()
    }
}
