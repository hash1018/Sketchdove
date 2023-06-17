use lib::figure::Figure;
use web_sys::MouseEvent;

use crate::components::draw_area::data::DrawAreaData;

use self::{line_mode::LineMode, select_mode::SelectMode};
use strum_macros::EnumIter;

use super::coordinates_converter::convert_device_to_figure;

pub mod line_mode;
pub mod pan_mode;
pub mod select_mode;

#[derive(PartialEq, Copy, Clone, Debug, EnumIter)]
pub enum DrawModeType {
    SelectMode,
    LineMode,
}

pub enum ShouldAction {
    Rerender,
    BackToSelect,
    AddFigure(Box<dyn Figure>),
}

pub trait DrawMode {
    fn mouse_press_event(
        &mut self,
        event: MouseEvent,
        data: &mut DrawAreaData,
    ) -> Option<ShouldAction>;
    fn mouse_mouse_event(
        &mut self,
        event: MouseEvent,
        data: &mut DrawAreaData,
    ) -> Option<ShouldAction>;
    fn mouse_release_event(
        &mut self,
        event: MouseEvent,
        data: &mut DrawAreaData,
    ) -> Option<ShouldAction>;
    fn get_type(&self) -> DrawModeType;

    fn convert_figure_coordinates(&self, event: &MouseEvent, data: &DrawAreaData) -> (f64, f64) {
        let x = event.offset_x() as f64;
        let y = event.offset_y() as f64;
        convert_device_to_figure(data.coordinates(), x, y)
    }
}

impl From<DrawModeType> for Box<dyn DrawMode> {
    fn from(val: DrawModeType) -> Self {
        let mode: Box<dyn DrawMode> = match val {
            DrawModeType::SelectMode => Box::new(SelectMode::new()),
            DrawModeType::LineMode => Box::new(LineMode::new()),
        };
        mode
    }
}
