use lib::figure::{line::Line, Rgba};
use web_sys::{MouseEvent, WebGlProgram, WebGlRenderingContext as GL};
use yew::{html, Component, Context};

use crate::algorithm::{
    coordinates_converter::convert_figure_to_webgl,
    draw_mode::{normal_mode::NormalMode, pan_mode::PanMode, DrawMode, DrawModeType},
    visitor::{drawer::Drawer, Accepter},
};

use self::data::{DrawAreaData, WebGLData};

pub mod data;

pub enum DrawAreaMessage {
    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
    MouseUp(MouseEvent),
}

pub struct DrawArea {
    data: DrawAreaData,
    current_mode: Box<dyn DrawMode>,
    webgl_data: Option<WebGLData>,
}

impl Component for DrawArea {
    type Message = DrawAreaMessage;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let data = DrawAreaData::new();
        let current_mode = NormalMode::new();
        DrawArea {
            data,
            current_mode: Box::new(current_mode),
            webgl_data: None,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let canvas = self.data.convert_canvas();
        let gl: GL = self.data.convert_gl_context();

        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
        gl.clear_color(209.0 / 255.0, 209.0 / 255.0, 209.0 / 255.0, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT);

        self.render_gl(gl);
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DrawAreaMessage::MouseDown(event) => {
                if event.button() == 1 {
                    self.current_mode = Box::new(PanMode::new());
                }
                self.current_mode.mouse_press_event(event, &mut self.data);
            }
            DrawAreaMessage::MouseMove(event) => {
                self.current_mode.mouse_mouse_event(event, &mut self.data);
            }
            DrawAreaMessage::MouseUp(event) => {
                self.current_mode.mouse_release_event(event, &mut self.data);
                if self.current_mode.get_type() == DrawModeType::PanMode {
                    self.current_mode = Box::new(NormalMode::new());
                }
                return false;
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let mousedown = ctx.link().callback(DrawAreaMessage::MouseDown);
        let mousemove = ctx.link().callback(DrawAreaMessage::MouseMove);
        let mouseup = ctx.link().callback(DrawAreaMessage::MouseUp);
        let node_ref_clone = self.data.node_ref();

        html! (
            <div style="width:100%; height:100%; overflow: hidden;">
                <canvas style="width:100%; height:100%;" onmousedown={mousedown} onmousemove={mousemove} onmouseup={mouseup} ref={node_ref_clone} />
            </div>
        )
    }
}

impl DrawArea {
    fn render_gl(&mut self, gl: GL) {
        if self.webgl_data.is_none() {
            self.webgl_data = Some(WebGLData::new(&gl).unwrap());
        }

        let rgba = Rgba::new(1.0, 0.0, 0.0, 1.0);

        let canvas = self.data.convert_canvas();
        let (start_x, start_y) = convert_figure_to_webgl(
            self.data.coordinates(),
            canvas.client_width() as f64,
            canvas.client_height() as f64,
            -100.0,
            -100.0,
        );
        let (end_x, end_y) = convert_figure_to_webgl(
            self.data.coordinates(),
            canvas.client_width() as f64,
            canvas.client_height() as f64,
            0.0,
            0.0,
        );

        let shader_program = self.webgl_data.as_ref().unwrap().shader_program();

        let drawer = Drawer::new(&gl, shader_program);

        let mut line = Line::new(start_x, start_y, end_x, end_y, rgba);

        line.accept(&drawer);
    }
}

#[allow(clippy::too_many_arguments)]
fn _draw_triangle(
    gl: &GL,
    shader_program: &WebGlProgram,
    x: f32,
    y: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    rgba: &Rgba,
) {
    let vectices: Vec<f32> = vec![x, y, x2, y2, x3, y3];
    let verts = js_sys::Float32Array::from(vectices.as_slice());
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

    let color = gl.get_uniform_location(shader_program, "color");
    gl.uniform4f(color.as_ref(), rgba.r, rgba.g, rgba.b, rgba.a);

    gl.draw_arrays(GL::TRIANGLES, 0, 3);
}
