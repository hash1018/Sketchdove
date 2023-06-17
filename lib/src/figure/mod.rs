use std::any::Any;

use as_dyn_trait::as_dyn_trait;

use self::leaf::line::Line;

pub mod composite;
pub mod leaf;

#[derive(Default, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }
}

pub trait Visitor {
    fn visit_line(&self, figure: &mut Line);
}

#[as_dyn_trait]
pub trait Figure {
    fn accept(&mut self, visitor: &dyn Visitor);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
