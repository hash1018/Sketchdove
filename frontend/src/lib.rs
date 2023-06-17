use std::{cell::RefCell, rc::Rc};

use lib::figure::Figure;

pub mod algorithm;
pub mod client;
pub mod components;
pub mod pages;

#[derive(Default)]
pub struct Coordinates {
    pub scroll_v_pos: f64,
    pub scroll_h_pos: f64,
    pub zoom_rate: f64,
    pub center_x: f64,
    pub center_y: f64,
}

impl Coordinates {
    pub fn new() -> Coordinates {
        Self {
            scroll_v_pos: 0.0,
            scroll_h_pos: 0.0,
            zoom_rate: 1.0,
            center_x: 100.0,
            center_y: 100.0,
        }
    }
}

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

    pub fn list(&self) -> Rc<RefCell<Vec<Box<dyn Figure>>>> {
        self.list.clone()
    }
}
