use lib::figure::line::Line;
pub mod drawer;

pub trait Visitor {
    fn visit_line(&self, figure: &mut Line);
}

pub trait Accepter {
    fn accept(&mut self, visitor: &dyn Visitor);
}

impl Accepter for Line {
    fn accept(&mut self, visitor: &dyn Visitor) {
        visitor.visit_line(self);
    }
}
