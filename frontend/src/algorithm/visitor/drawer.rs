use lib::figure::Visitor;
use web_sys::{CanvasRenderingContext2d, WebGlProgram, WebGlRenderingContext};

pub struct Drawer<'a> {
    context: &'a CanvasRenderingContext2d,
}

impl<'a> Drawer<'a> {
    pub fn new(context: &'a CanvasRenderingContext2d) -> Self {
        Self { context }
    }
}

impl Visitor for Drawer<'_> {
    fn visit_line(&self, line: &mut lib::figure::line::Line) {
        self.context.begin_path();
        self.context.move_to(line.start_x(), line.start_y());
        self.context.line_to(line.end_x(), line.end_y());
        self.context.stroke();
    }
}

pub struct DrawerGL<'a> {
    gl: &'a WebGlRenderingContext,
    shader_program: &'a WebGlProgram,
}

impl<'a> DrawerGL<'a> {
    pub fn new(gl: &'a WebGlRenderingContext, shader_program: &'a WebGlProgram) -> Self {
        Self { gl, shader_program }
    }
}

impl Visitor for DrawerGL<'_> {
    fn visit_line(&self, line: &mut lib::figure::line::Line) {
        let vectices: Vec<f32> = vec![
            line.start_x() as f32,
            line.start_y() as f32,
            line.end_x() as f32,
            line.end_y() as f32,
        ];
        let verts = js_sys::Float32Array::from(vectices.as_slice());
        self.gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &verts,
            WebGlRenderingContext::STATIC_DRAW,
        );

        let color = self.gl.get_uniform_location(self.shader_program, "color");
        let rgba = line.color();
        self.gl
            .uniform4f(color.as_ref(), rgba.r, rgba.g, rgba.b, rgba.a);

        self.gl.draw_arrays(WebGlRenderingContext::LINES, 0, 2);
    }
}
