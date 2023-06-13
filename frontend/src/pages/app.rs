use std::sync::Mutex;
use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

use crate::pages::{main::Main, workspace::Workspace};
use once_cell::sync::Lazy;

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

static USER_NAME: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

pub fn set_user_name(user_name: Option<String>) {
    *USER_NAME.lock().unwrap() = user_name;
}

pub fn user_name() -> Option<String> {
    let user_name = USER_NAME.lock().unwrap().clone();
    user_name
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
