use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext as GL};
use yew::NodeRef;

use crate::Coordinates;

#[derive(Default)]
pub struct DrawAreaData {
    node_ref: NodeRef,
    coordinates: Coordinates,
}

impl DrawAreaData {
    pub fn new() -> Self {
        Self {
            node_ref: NodeRef::default(),
            coordinates: Coordinates::new(),
        }
    }

    pub fn convert_canvas(&self) -> HtmlCanvasElement {
        self.node_ref.cast::<HtmlCanvasElement>().unwrap()
    }

    pub fn convert_gl_context(&self) -> GL {
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap()
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }

    pub fn node_ref(&self) -> NodeRef {
        self.node_ref.clone()
    }

    pub fn set_scroll_pos(&mut self, h_pos: f64, v_pos: f64) {
        self.coordinates.scroll_h_pos = h_pos;
        self.coordinates.scroll_v_pos = v_pos;
    }
}
