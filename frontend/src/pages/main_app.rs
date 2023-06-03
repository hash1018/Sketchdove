use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::login::Login;
use crate::pages::workspace::Workspace;

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
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
