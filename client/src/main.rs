use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::login::Login;
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
