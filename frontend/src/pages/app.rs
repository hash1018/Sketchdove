use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

use crate::pages::{main::Main, workspace::Workspace};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Main,
    #[at("/:id")]
    Workspace { id: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Main => {
            html! { <Main /> }
        }
        Route::Workspace { id } => {
            html! { <Workspace {id} /> }
        }
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
