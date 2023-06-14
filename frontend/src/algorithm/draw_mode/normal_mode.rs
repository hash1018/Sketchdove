use crate::components::draw_area::data::DrawAreaData;

use super::{DrawMode, ShouldAction};

#[derive(Default)]
pub struct NormalMode {}

impl NormalMode {
    pub fn new() -> Self {
        NormalMode {}
    }
}

impl DrawMode for NormalMode {
    fn mouse_press_event(
        &mut self,
        _event: web_sys::MouseEvent,
        _data: &mut DrawAreaData,
    ) -> Option<ShouldAction> {
        None
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
        super::DrawModeType::NormalMode
    }
}
