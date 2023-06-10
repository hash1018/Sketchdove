use lib::figure::Rgba;
use web_sys::{MouseEvent, WebGlProgram, WebGlRenderingContext as GL};
use yew::{html, Component, Context};

use crate::algorithm::{
    coordinates_converter::convert_figure_to_webgl,
    draw_mode::{normal_mode::NormalMode, pan_mode::PanMode, DrawMode, DrawModeType},
};

use self::data::DrawAreaData;

pub mod data;

pub enum DrawAreaMessage {
    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
    MouseUp(MouseEvent),
}

pub struct DrawArea {
    data: DrawAreaData,
    current_mode: Option<Box<dyn DrawMode>>,
}

impl Component for DrawArea {
    type Message = DrawAreaMessage;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let data = DrawAreaData::new();
        let current_mode = NormalMode::new();
        DrawArea {
            data,
            current_mode: Some(Box::new(current_mode)),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let canvas = self.data.convert_canvas();
        let gl: GL = self.data.convert_gl_context();

        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

        self.render_gl(gl);
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DrawAreaMessage::MouseDown(event) => {
                if event.button() == 1 {
                    self.current_mode = Some(Box::new(PanMode::new()));
                }

                if let Some(mut current_mode) = self.current_mode.take() {
                    current_mode.mouse_press_event(event, &mut self.data);
                    self.current_mode = Some(current_mode);
                }
            }
            DrawAreaMessage::MouseMove(event) => {
                if let Some(mut current_mode) = self.current_mode.take() {
                    current_mode.mouse_mouse_event(event, &mut self.data);
                    self.current_mode = Some(current_mode);
                }
            }
            DrawAreaMessage::MouseUp(event) => {
                if let Some(mut current_mode) = self.current_mode.take() {
                    current_mode.mouse_release_event(event, &mut self.data);

                    if current_mode.get_type() == DrawModeType::PanMode {
                        self.current_mode = Some(Box::new(NormalMode::new()));
                    } else {
                        self.current_mode = Some(current_mode);
                    }
                }
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
    fn render_gl(&self, gl: GL) {
        let vert_code = "precision mediump float;

        attribute vec2 a_position;

        void main() {
            gl_Position = vec4(a_position, 0.0, 1.0);
        }";

        let frag_code = "precision mediump float;

        uniform vec4 color;

        void main() {
            gl_FragColor = color;
        }";

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        let vertex_buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));

        let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

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

        draw_line(
            &gl,
            &shader_program,
            start_x as f32,
            start_y as f32,
            end_x as f32,
            end_y as f32,
            &rgba,
        );

        /*
        draw_triangle(
            &gl,
            &shader_program,
            -1.0,
            -1.0,
            1.0,
            -1.0,
            -1.0,
            1.0,
            &rgba,
        );
        */
    }
}

fn draw_line(
    gl: &GL,
    shader_program: &WebGlProgram,
    x: f32,
    y: f32,
    x2: f32,
    y2: f32,
    rgba: &Rgba,
) {
    let vectices: Vec<f32> = vec![x, y, x2, y2];
    let verts = js_sys::Float32Array::from(vectices.as_slice());
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

    let color = gl.get_uniform_location(shader_program, "color");
    gl.uniform4f(color.as_ref(), rgba.r, rgba.g, rgba.b, rgba.a);

    gl.draw_arrays(GL::LINES, 0, 2);
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
