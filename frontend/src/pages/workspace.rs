use std::rc::Rc;

use lib::message::ServerMessage;
use yew::{html, Component};
use yew_agent::{Bridge, Bridged};

use crate::client::{event_bus::EventBus, WebsocketService};

pub enum WorkSpaceMessage {
    HandleServerMessage(ServerMessage),
}

pub struct Workspace {
    _wss: WebsocketService,
    _event_bus: Box<dyn Bridge<EventBus>>,
}

impl Component for Workspace {
    type Message = WorkSpaceMessage;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let wss = WebsocketService::new();
        wss.connect();
        let callback = {
            let link = ctx.link().clone();
            move |e| link.send_message(WorkSpaceMessage::HandleServerMessage(e))
        };
        Workspace {
            _wss: wss,
            _event_bus: EventBus::bridge(Rc::new(callback)),
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WorkSpaceMessage::HandleServerMessage(server_message) => {
                log::debug!("received message from event_bus {server_message:?}");
            }
        }
        true
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <body> { "Hello workspace" } </body>
        }
    }
}
