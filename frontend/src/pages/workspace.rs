use std::rc::Rc;

use lib::message::ServerMessage;
use yew::{html, Component, Context, Properties};
use yew_agent::{Bridge, Bridged};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    client::{event_bus::EventBus, websocket_service::WebsocketService},
    components::{chat::Chat, title_bar::TitleBar},
    pages::app::user_name,
};

use super::app::Route;

pub enum WorkSpaceMessage {
    HandleServerMessage(ServerMessage),
    HandleChildRequest(ChildRequestType),
    RequestInit,
}

pub enum ChildRequestType {
    Leave,
    ShowChat(bool),
}

#[derive(Clone, PartialEq, Properties)]
pub struct WorkspaceProps {
    pub id: String,
}

pub struct Workspace {
    wss: Option<WebsocketService>,
    _event_bus: Option<Box<dyn Bridge<EventBus>>>,
    show_chat: bool,
}

impl Component for Workspace {
    type Message = WorkSpaceMessage;
    type Properties = WorkspaceProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let user_name = user_name();
        if user_name.is_some() {
            let link = ctx.link();
            link.send_message(WorkSpaceMessage::RequestInit);
        }

        Self {
            wss: None,
            _event_bus: None,
            show_chat: false,
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(wss) = self.wss.as_ref() {
            wss.disconnect();
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WorkSpaceMessage::RequestInit => {
                (self.wss, self._event_bus) = init(ctx);
                return false;
            }
            WorkSpaceMessage::HandleServerMessage(server_message) => {
                log::debug!("received message from event_bus {server_message:?}");
            }
            WorkSpaceMessage::HandleChildRequest(request) => match request {
                ChildRequestType::Leave => {
                    let navigator = ctx.link().navigator().unwrap();
                    navigator.push(&Route::Main);
                }
                ChildRequestType::ShowChat(show) => {
                    self.show_chat = show;
                }
            },
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let handler = ctx.link().callback(WorkSpaceMessage::HandleChildRequest);
        let show_chat = self.show_chat;

        html! {
            <body>
                <div class="top"> <TitleBar {handler} {show_chat} /> </div>
                <div class="content">
                    <div class="left"></div>
                    if show_chat {
                        <div class="chat_position"> <Chat /> </div>
                    }
                </div>
            </body>
        }
    }
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
