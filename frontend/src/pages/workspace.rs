use std::rc::Rc;

use lib::{message::ServerMessage, user::User};
use wasm_bindgen_futures::spawn_local;
use yew::{html, Component, Context};
use yew_agent::{Bridge, Bridged};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    api::user_api::{api_check_login_valid, api_logout_user, api_register_user},
    client::{event_bus::EventBus, websocket_service::WebsocketService},
    components::{draw_area::DrawArea, title_bar::TitleBar},
    pages::main_app::Route,
};

pub enum WorkSpaceMessage {
    HandleServerMessage(ServerMessage),
    HandleChildRequest(String),
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
}

impl Component for Workspace {
    type Message = WorkSpaceMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        check_login_valid(ctx);
        Workspace {
            wss: None,
            _event_bus: None,
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
            WorkSpaceMessage::HandleChildRequest(message) => match message.as_str() {
                "logout" => {
                    let navigator = ctx.link().navigator().unwrap();
                    let user = User::new("name".to_string());
                    spawn_local(async move {
                        if let Ok(()) = api_logout_user(&user).await {
                            navigator.replace(&Route::Login);
                        }
                    });
                }
                "register" => {
                    let user = User::new("name".to_string());
                    spawn_local(async move {
                        api_register_user(&user).await.unwrap();
                    });
                }
                _ => {}
            },
            WorkSpaceMessage::RequestInit => {
                init(ctx);
                return true;
            }
        }
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let handler = ctx.link().callback(WorkSpaceMessage::HandleChildRequest);
        html! {
            <body>
                <div class="top"> <TitleBar {handler}/> </div>
                <div class="content">
                    <div class="left"></div>
                    <div class="center"> <DrawArea /> </div>
                    <div class="right"></div>
                </div>
            </body>
        }
    }
}
