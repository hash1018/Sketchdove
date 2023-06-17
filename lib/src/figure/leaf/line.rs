use crate::figure::{Figure, Rgba, Visitor};

use super::Leaf;

pub struct Line {
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
    color: Rgba,
}

impl Leaf for Line {}

impl Figure for Line {
    fn accept(&mut self, visitor: &dyn Visitor) {
        visitor.visit_line(self);
    }
}

impl Line {
    pub fn new(start_x: f64, start_y: f64, end_x: f64, end_y: f64, color: Rgba) -> Self {
        Self {
            start_x,
            start_y,
            end_x,
            end_y,
            color,
        }
    }

    pub fn start_x(&self) -> f64 {
        self.start_x
    }

    pub fn start_y(&self) -> f64 {
        self.start_y
    }

    pub fn end_x(&self) -> f64 {
        self.end_x
    }

    pub fn end_y(&self) -> f64 {
        self.end_y
    }

    pub fn color(&self) -> Rgba {
        self.color
    }
}
