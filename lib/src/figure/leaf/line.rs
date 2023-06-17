use crate::figure::{Color, Figure, Visitor};

use super::Leaf;

pub struct Line {
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
    color: Color,
}

impl Leaf for Line {}

impl Figure for Line {
    fn accept(&mut self, visitor: &dyn Visitor) {
        visitor.visit_line(self);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Line {
    pub fn new(start_x: f64, start_y: f64, end_x: f64, end_y: f64, color: Color) -> Self {
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

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_start_x(&mut self, start_x: f64) {
        self.start_x = start_x;
    }

    pub fn set_start_y(&mut self, start_y: f64) {
        self.start_y = start_y;
    }

    pub fn set_end_x(&mut self, end_x: f64) {
        self.end_x = end_x;
    }

    pub fn set_end_y(&mut self, end_y: f64) {
        self.end_y = end_y;
    }
}
