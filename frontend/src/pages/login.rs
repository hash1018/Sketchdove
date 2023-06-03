use std::rc::Rc;

use lib::message::ClientMessage;
use lib::message::ServerMessage;
use yew::html;
use yew::prelude::*;
use yew_agent::Bridge;
use yew_agent::Bridged;
//use yew_router::prelude::*;

use crate::client::event_bus::EventBus;
use crate::client::Client;
//use crate::Route;

pub enum Message {
    LoginButtonClicked,
    HandleServerMessage(ServerMessage),
}

pub struct Login {
    client: Client,
    _event_bus: Box<dyn Bridge<EventBus>>,
}

impl Component for Login {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let client = Client::new();
        let callback = {
            let link = ctx.link().clone();
            move |e| link.send_message(Message::HandleServerMessage(e))
        };

        Self {
            client,
            _event_bus: EventBus::bridge(Rc::new(callback)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::LoginButtonClicked => {
                //let navigator = ctx.link().navigator().unwrap();
                //navigator.push(&Route::Workspace);
                if !self.client.is_connected() {
                    self.client.connect();
                }
                self.client.send_message_to_server(ClientMessage::Test);
            }
            Message::HandleServerMessage(server_message) => {
                log::debug!("received message from event_bus {server_message:?}");
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let login_button_clicked = ctx.link().callback(|_| Message::LoginButtonClicked);

        html!(
            <body>
                <div class="center">
                    <input id="username" style="display:block; width:100px; box-sizing: border-box" type="text" placeholder="username" />
                    <button onclick={login_button_clicked}> {"Login"} </button>
                </div>
            </body>
        )
    }
}
