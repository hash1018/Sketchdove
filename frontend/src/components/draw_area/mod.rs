use lib::figure::{line::Line, Rgba};
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, WebGlProgram,
    WebGlRenderingContext as GL,
};
use yew::{html, Callback, Component, Context, Properties};

use crate::{
    algorithm::{
        coordinates_converter::{convert_figure_to_device, convert_figure_to_webgl},
        draw_mode::{
            normal_mode::NormalMode, pan_mode::PanMode, DrawMode, DrawModeType, ShouldAction,
        },
        visitor::{
            drawer::{Drawer, DrawerGL},
            Accepter,
        },
    },
    pages::workspace::ChildRequestType,
};

use self::data::{DrawAreaData, WebGLData};

pub mod data;

pub enum DrawAreaMessage {
    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
    MouseUp(MouseEvent),
}

#[derive(Clone, PartialEq, Properties)]
pub struct DrawAreaProps {
    pub handler: Callback<ChildRequestType>,
    pub current_mode: DrawModeType,
}

pub struct DrawArea {
    data: DrawAreaData,
    current_mode: Box<dyn DrawMode>,
    pan_mode: Option<PanMode>,
    webgl_data: Option<WebGLData>,
}

impl Component for DrawArea {
    type Message = DrawAreaMessage;
    type Properties = DrawAreaProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let data = DrawAreaData::new();
        let current_mode = NormalMode::new();
        DrawArea {
            data,
            current_mode: Box::new(current_mode),
            pan_mode: None,
            webgl_data: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let new_mode = ctx.props().current_mode;
        let old_mode = old_props.current_mode;
        if new_mode != old_mode {
            self.current_mode = new_mode.into();
        }
        false
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let canvas = self.data.convert_canvas();
        let context = self.data.convert_2d_context();
        self.render_2d_context(canvas, context);

        //let canvas = self.data.convert_canvas();
        //let gl: GL = self.data.convert_gl_context();
        //self.render_gl(gl, canvas);
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let should_action = match msg {
            DrawAreaMessage::MouseDown(event) => {
                if event.button() == 1 {
                    let mut pan_mode = PanMode::new();
                    let should_action = pan_mode.mouse_press_event(event, &mut self.data);
                    self.pan_mode = Some(pan_mode);
                    should_action
                } else {
                    self.current_mode.mouse_press_event(event, &mut self.data)
                }
            }
            DrawAreaMessage::MouseMove(event) => {
                if let Some(mut pan_mode) = self.pan_mode.take() {
                    let should_action = pan_mode.mouse_mouse_event(event, &mut self.data);
                    self.pan_mode = Some(pan_mode);
                    should_action
                } else {
                    self.current_mode.mouse_mouse_event(event, &mut self.data)
                }
            }
            DrawAreaMessage::MouseUp(event) => {
                if self.pan_mode.take().is_none() {
                    self.current_mode.mouse_release_event(event, &mut self.data)
                } else {
                    None
                }
            }
        };

        if let Some(should_action) = should_action {
            match should_action {
                ShouldAction::BackToNormal => {
                    ctx.props()
                        .handler
                        .emit(ChildRequestType::ChangeMode(DrawModeType::NormalMode));
                }
                ShouldAction::Rerender => {
                    return true;
                }
            }
        }
        false
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
    fn render_2d_context(&self, canvas: HtmlCanvasElement, context: CanvasRenderingContext2d) {
        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);
        context.clear_rect(
            0.0,
            0.0,
            canvas.client_width() as f64,
            canvas.client_height() as f64,
        );

        let rgba = Rgba::new(1.0, 0.0, 0.0, 1.0);

        let (start_x, start_y) = convert_figure_to_device(self.data.coordinates(), -100.0, 100.0);
        let (end_x, end_y) = convert_figure_to_device(self.data.coordinates(), 0.0, 0.0);

        let mut line = Line::new(
            start_x as f64,
            start_y as f64,
            end_x as f64,
            end_y as f64,
            rgba,
        );

        let drawer = Drawer::new(&context);

        line.accept(&drawer);
    }

    #[allow(dead_code)]
    fn render_gl(&mut self, gl: GL, canvas: HtmlCanvasElement) {
        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
        gl.clear_color(209.0 / 255.0, 209.0 / 255.0, 209.0 / 255.0, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT);

        if self.webgl_data.is_none() {
            self.webgl_data = Some(WebGLData::new(&gl).unwrap());
        }

        let rgba = Rgba::new(1.0, 0.0, 0.0, 1.0);

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

        let drawer = DrawerGL::new(&gl, shader_program);

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
