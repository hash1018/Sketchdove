use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::login::Login;
use crate::pages::register::Register;
use crate::pages::workspace::Workspace;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/register")]
    Register,
    #[at("/workspace")]
    Workspace,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Login => {
            html! { <Login /> }
        }
        Route::Register => {
            html! { <Register /> }
        }
        Route::Workspace => {
            html! { <Workspace /> }
        }
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
