use web_sys::MouseEvent;

use crate::components::draw_area::data::DrawAreaData;

use self::{line_mode::LineMode, normal_mode::NormalMode};

pub mod line_mode;
pub mod normal_mode;
pub mod pan_mode;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum DrawModeType {
    NormalMode,
    LineMode,
}
pub trait DrawMode {
    fn mouse_press_event(&mut self, event: MouseEvent, data: &mut DrawAreaData);
    fn mouse_mouse_event(&mut self, event: MouseEvent, data: &mut DrawAreaData);
    fn mouse_release_event(&mut self, event: MouseEvent, data: &mut DrawAreaData) -> bool;
    fn get_type(&self) -> DrawModeType;
}

impl From<DrawModeType> for Box<dyn DrawMode> {
    fn from(val: DrawModeType) -> Self {
        let mode: Box<dyn DrawMode> = match val {
            DrawModeType::NormalMode => Box::new(NormalMode::new()),
            DrawModeType::LineMode => Box::new(LineMode::new()),
        };
        mode
    }
}
