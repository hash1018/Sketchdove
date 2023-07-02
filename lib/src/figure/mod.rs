use std::any::Any;

use as_dyn_trait::as_dyn_trait;
use serde::{Deserialize, Serialize};

use self::leaf::line::{Line, LineData};

pub mod composite;
pub mod leaf;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
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
    fn data(&self) -> FigureData;
}

impl From<FigureData> for Box<dyn Figure> {
    fn from(val: FigureData) -> Self {
        let figure: Box<dyn Figure> = match val {
            FigureData::Line(data) => Box::new(Line::new(
                data.start_x,
                data.start_y,
                data.end_x,
                data.end_y,
                data.color,
            )),
        };
        figure
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FigureData {
    Line(LineData),
}
