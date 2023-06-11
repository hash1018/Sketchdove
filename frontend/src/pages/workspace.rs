use std::rc::Rc;

use lib::{message::ServerMessage, user::User};
use wasm_bindgen_futures::spawn_local;
use yew::{html, Component, Context};
use yew_agent::{Bridge, Bridged};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    algorithm::draw_mode::DrawModeType,
    api::user_api::{api_check_login_valid, api_logout_user},
    client::{event_bus::EventBus, websocket_service::WebsocketService},
    components::{draw_area::DrawArea, title_bar::TitleBar},
    pages::main_app::Route,
};

pub enum WorkSpaceMessage {
    HandleServerMessage(ServerMessage),
    HandleChildRequest(ChildRequestType),
    RequestInit,
}

pub enum ChildRequestType {
    Logout,
    ChangeMode(DrawModeType),
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
    current_mode: DrawModeType,
}

impl Component for Workspace {
    type Message = WorkSpaceMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        check_login_valid(ctx);
        Workspace {
            wss: None,
            _event_bus: None,
            current_mode: DrawModeType::NormalMode,
        }
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
            WorkSpaceMessage::HandleChildRequest(request_type) => match request_type {
                ChildRequestType::Logout => {
                    let navigator = ctx.link().navigator().unwrap();
                    let user = User::new("name".to_string());
                    spawn_local(async move {
                        if let Ok(()) = api_logout_user(&user).await {
                            navigator.replace(&Route::Login);
                        }
                    });
                }
                ChildRequestType::ChangeMode(mode) => {
                    if mode != self.current_mode {
                        self.current_mode = mode;
                        return true;
                    }
                }
            },
            WorkSpaceMessage::RequestInit => {
                (self.wss, self._event_bus) = init(ctx);
                return false;
            }
        }
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let handler = ctx.link().callback(WorkSpaceMessage::HandleChildRequest);
        let current_mode = self.current_mode;
        let handler_clone = handler.clone();
        html! {
            <body>
                <div class="top"> <TitleBar {handler} {current_mode} /> </div>
                <div class="content">
                    <div class="left"></div>
                    <div class="center"> <DrawArea handler = {handler_clone} {current_mode} /> </div>
                    <div class="right"></div>
                </div>
            </body>
        }
    }
}
