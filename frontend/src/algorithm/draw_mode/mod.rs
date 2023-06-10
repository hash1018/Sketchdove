use web_sys::MouseEvent;

use crate::components::draw_area::data::DrawAreaData;

pub mod normal_mode;
pub mod pan_mode;

#[derive(PartialEq)]
pub enum DrawModeType {
    NormalMode,
    PanMode,
}

pub trait DrawMode {
    fn mouse_press_event(&mut self, event: MouseEvent, data: &mut DrawAreaData);
    fn mouse_mouse_event(&mut self, event: MouseEvent, data: &mut DrawAreaData);
    fn mouse_release_event(&mut self, event: MouseEvent, data: &mut DrawAreaData);
    fn get_type(&self) -> DrawModeType;
}
