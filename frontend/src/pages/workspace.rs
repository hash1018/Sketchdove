use std::rc::Rc;

use lib::{figure::Rgba, message::ServerMessage, user::User};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext as GL};
use yew::{html, Component, Context, NodeRef};
use yew_agent::{Bridge, Bridged};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    api::user_api::{api_check_login_valid, api_logout_user, api_register_user},
    client::{event_bus::EventBus, websocket_service::WebsocketService},
    pages::main_app::Route,
};

pub enum WorkSpaceMessage {
    HandleServerMessage(ServerMessage),
    LogoutButtonClicked,
    RegisterButtonClicked,
    RequestInit,
}

fn check_login_valid(ctx: &Context<Workspace>) {
    let navigator = ctx.link().navigator().unwrap();
    let link = ctx.link().clone();
    spawn_local(async move {
        if let Ok(()) = api_check_login_valid().await {
            link.send_message(WorkSpaceMessage::RequestInit);
        } else {
            navigator.replace(&Route::Login);
        }
    });
}

fn init(ctx: &Context<Workspace>) -> (Option<WebsocketService>, Option<Box<dyn Bridge<EventBus>>>) {
    let wss = WebsocketService::new();
    wss.connect().unwrap();
    let callback = {
        let link = ctx.link().clone();
        move |e| link.send_message(WorkSpaceMessage::HandleServerMessage(e))
    };

    (Some(wss), Some(EventBus::bridge(Rc::new(callback))))
}

pub struct Workspace {
    wss: Option<WebsocketService>,
    _event_bus: Option<Box<dyn Bridge<EventBus>>>,
    node_ref: NodeRef,
}

impl Component for Workspace {
    type Message = WorkSpaceMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        check_login_valid(ctx);
        Workspace {
            wss: None,
            _event_bus: None,
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

        self.render_gl(gl);
    }

    fn destroy(&mut self, _ctx: &yew::Context<Self>) {
        if let Some(wss) = self.wss.as_ref() {
            wss.disconnect();
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WorkSpaceMessage::HandleServerMessage(server_message) => {
                log::debug!("received message from event_bus {server_message:?}");
            }
            WorkSpaceMessage::LogoutButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                let user = User::new("name".to_string());
                spawn_local(async move {
                    if let Ok(()) = api_logout_user(&user).await {
                        navigator.replace(&Route::Login);
                    }
                });
            }
            WorkSpaceMessage::RegisterButtonClicked => {
                let user = User::new("name".to_string());
                spawn_local(async move {
                    api_register_user(&user).await.unwrap();
                });
            }
            WorkSpaceMessage::RequestInit => {
                init(ctx);
                return true;
            }
        }
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let logout_button_clicked = ctx
            .link()
            .callback(|_| WorkSpaceMessage::LogoutButtonClicked);
        let register_button_clicked = ctx
            .link()
            .callback(|_| WorkSpaceMessage::RegisterButtonClicked);

        html!(
            <body>
                <div class="center">
                    <input id="username" style="display:block; width:100px; box-sizing: border-box" type="text" placeholder="username" />
                    <button onclick={logout_button_clicked}> {"Logout"} </button>
                    <button onclick={register_button_clicked}> {"Register"} </button>
                </div>
                <div>
                    <canvas ref={self.node_ref.clone()} />
                </div>
            </body>
        )
    }
}

impl Workspace {
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
fn draw_triangle(
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
