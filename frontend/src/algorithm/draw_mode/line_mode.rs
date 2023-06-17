use lib::figure::{leaf::line::Line, Rgba};

use crate::{
    algorithm::coordinates_converter::convert_device_to_figure,
    components::draw_area::data::DrawAreaData,
};

use super::{DrawMode, ShouldAction};

#[derive(Default)]
pub struct LineMode {
    start_x: Option<f64>,
    start_y: Option<f64>,
}

impl LineMode {
    pub fn new() -> Self {
        LineMode {
            start_x: None,
            start_y: None,
        }
    }
}

impl DrawMode for LineMode {
    fn mouse_press_event(
        &mut self,
        event: web_sys::MouseEvent,
        data: &mut DrawAreaData,
    ) -> Option<ShouldAction> {
        let x = event.offset_x() as f64;
        let y = event.offset_y() as f64;
        let (x, y) = convert_device_to_figure(data.coordinates(), x, y);

        if let (Some(start_x), Some(start_y)) = (self.start_x.take(), self.start_y.take()) {
            let line = Line::new(start_x, start_y, x, y, Rgba::new(0.0, 0.0, 0.0, 1.0));
            Some(ShouldAction::AddFigure(Box::new(line)))
        } else {
            self.start_x = Some(x);
            self.start_y = Some(y);
            None
        }
    }

    fn mouse_mouse_event(
        &mut self,
        _event: web_sys::MouseEvent,
        _data: &mut DrawAreaData,
    ) -> Option<ShouldAction> {
        None
    }

    fn mouse_release_event(
        &mut self,
        _event: web_sys::MouseEvent,
        _data: &mut DrawAreaData,
    ) -> Option<ShouldAction> {
        None
    }

    fn get_type(&self) -> super::DrawModeType {
        super::DrawModeType::LineMode
    }
}
