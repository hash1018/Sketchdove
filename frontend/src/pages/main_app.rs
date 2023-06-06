use std::rc::Rc;
use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::login::Login;
use crate::pages::register::Register;
use crate::pages::workspace::Workspace;
use crate::LoginedUser;

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
    let logined_user = use_state(|| Rc::new(LoginedUser::new()));

    html! {
        <ContextProvider<Rc<LoginedUser>> context={(*logined_user).clone()}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<Rc<LoginedUser>>>
    }
}
