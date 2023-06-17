use self::line::Line;

pub mod line;

#[derive(Default, Copy, Clone)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Rgba { r, g, b, a }
    }
}

pub trait Visitor {
    fn visit_line(&self, figure: &mut Line);
}

pub trait Figure {
    fn accept(&mut self, visitor: &dyn Visitor);
}
