use yew::html;
use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::workspace::Workspace;

mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Login,
    #[at("/workspace")]
    Workspace,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Login => {
            html! { <Login /> }
        }
        Route::Workspace => {
            html! { <Workspace /> }
        }
    }
}

pub enum Message {
    LoginButtonClicked,
}

pub struct Login {}

impl Component for Login {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::LoginButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::Workspace);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!({ self.view_navi(ctx.link()) })
    }
}

impl Login {
    fn view_navi(&self, link: &Scope<Self>) -> Html {
        let login_button_clicked = link.callback(|_| Message::LoginButtonClicked);

        html!(
            <body>
                <div class="center">
                    <input id="username" style="display:block; width:100px; box-sizing: border-box" type="text" placeholder="username" />
                    <button onclick={login_button_clicked}> {"Login"} </button>
                </div>
            </body>
        )
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<Main>::new().render();
}
