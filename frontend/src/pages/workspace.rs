use std::rc::Rc;

use lib::{message::ServerMessage, user::User};
use wasm_bindgen_futures::spawn_local;
use yew::{html, Component, ContextHandle};
use yew_agent::{Bridge, Bridged};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    api::user_api::{api_logout_user, api_register_user},
    client::{event_bus::EventBus, websocket_service::WebsocketService},
    pages::main_app::Route,
    LoginedUser,
};

pub enum WorkSpaceMessage {
    HandleServerMessage(ServerMessage),
    LogoutButtonClicked,
    RegisterButtonClicked,
    ContextChanged(Rc<LoginedUser>),
}

pub struct Workspace {
    wss: WebsocketService,
    _event_bus: Box<dyn Bridge<EventBus>>,
    logined_user: Rc<LoginedUser>,
    _listener: ContextHandle<Rc<LoginedUser>>,
}

impl Component for Workspace {
    type Message = WorkSpaceMessage;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let wss = WebsocketService::new();
        wss.connect().unwrap();
        let callback = {
            let link = ctx.link().clone();
            move |e| link.send_message(WorkSpaceMessage::HandleServerMessage(e))
        };

        let (logined_user, _listener) = ctx
            .link()
            .context::<Rc<LoginedUser>>(ctx.link().callback(WorkSpaceMessage::ContextChanged))
            .unwrap();

        Workspace {
            wss,
            _event_bus: EventBus::bridge(Rc::new(callback)),
            logined_user,
            _listener,
        }
    }

    fn destroy(&mut self, _ctx: &yew::Context<Self>) {
        self.wss.disconnect();
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WorkSpaceMessage::HandleServerMessage(server_message) => {
                log::debug!("received message from event_bus {server_message:?}");
            }
            WorkSpaceMessage::LogoutButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                let logined_user = self.logined_user.clone();
                let user = logined_user.user();
                spawn_local(async move {
                    if let Some(user) = user {
                        if let Ok(()) = api_logout_user(&user).await {
                            logined_user.logout();
                            log::info!("logout done");
                            navigator.push(&Route::Login);
                        }
                    }
                });
            }
            WorkSpaceMessage::RegisterButtonClicked => {
                let is_logined = self.logined_user.is_logined();
                log::info!("is_logined? {is_logined}");
                let user = User::new("name".to_string());
                spawn_local(async move {
                    api_register_user(&user).await.unwrap();
                    log::info!("return after calling api register user {user:?}");
                });
            }
            WorkSpaceMessage::ContextChanged(logined_user) => {
                self.logined_user = logined_user;
                return false;
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let logout_button_clicked = ctx
            .link()
            .callback(|_| WorkSpaceMessage::LogoutButtonClicked);
        let register_button_clicked = ctx
            .link()
            .callback(|_| WorkSpaceMessage::RegisterButtonClicked);

        html!(
            <body>
                <div class="center">
                    <input id="username" style="display:block; width:100px; box-sizing: border-box" type="text" placeholder="username" />
                    <button onclick={logout_button_clicked}> {"Logout"} </button>
                    <button onclick={register_button_clicked}> {"Register"} </button>
                </div>
            </body>
        )
    }
}
