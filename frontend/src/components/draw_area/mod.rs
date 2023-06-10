use lib::figure::Rgba;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext as GL};
use yew::{html, Component, Context, NodeRef};

pub struct DrawArea {
    node_ref: NodeRef,
}

impl Component for DrawArea {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        DrawArea {
            node_ref: NodeRef::default(),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

        self.render_gl(gl);
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! (
            <div style="width:100%; height:100%; overflow: hidden;">
                <canvas style="width:100%; height:100%;" ref={self.node_ref.clone()} />
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

        let rgba = Rgba::new(1.0, 1.0, 0.0, 1.0);
        draw_line(&gl, &shader_program, -1.0, -1.0, 1.0, 1.0, &rgba);

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
