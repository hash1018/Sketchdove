use std::sync::Mutex;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::AddEventListenerOptions;
use yew::{html, Component, Html};
use yew_router::{BrowserRouter, Routable, Switch};

use crate::pages::{main::Main, workspace::workspace::Workspace};
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

pub struct App {
    wheel_closure: Option<Closure<dyn FnMut(web_sys::WheelEvent)>>,
    contextmenu_closure: Option<Closure<dyn FnMut(web_sys::MouseEvent)>>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let wheel_closure = prevent_wheel_closure();
        let contextmenu_closure = prevent_contextmenu_closure();

        Self {
            wheel_closure,
            contextmenu_closure,
        }
    }

    fn destroy(&mut self, _ctx: &yew::Context<Self>) {
        if let Some(window) = web_sys::window() {
            if let Some(closure) = self.wheel_closure.take() {
                window
                    .remove_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
                    .unwrap();
            }

            if let Some(closure) = self.contextmenu_closure.take() {
                window
                    .remove_event_listener_with_callback(
                        "contextmenu",
                        closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    }
}

fn prevent_wheel_closure() -> Option<Closure<dyn FnMut(web_sys::WheelEvent)>> {
    if let Some(window) = web_sys::window() {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::WheelEvent| {
            event.prevent_default();
        });

        let mut option = AddEventListenerOptions::new();
        option.passive(false);

        window
            .add_event_listener_with_callback_and_add_event_listener_options(
                "wheel",
                closure.as_ref().unchecked_ref(),
                &option,
            )
            .unwrap();

        Some(closure)
    } else {
        None
    }
}

fn prevent_contextmenu_closure() -> Option<Closure<dyn FnMut(web_sys::MouseEvent)>> {
    if let Some(window) = web_sys::window() {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            event.prevent_default();
        });

        let mut option = AddEventListenerOptions::new();
        option.passive(false);

        window
            .add_event_listener_with_callback_and_add_event_listener_options(
                "contextmenu",
                closure.as_ref().unchecked_ref(),
                &option,
            )
            .unwrap();

        Some(closure)
    } else {
        None
    }
}
