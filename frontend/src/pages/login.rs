use lib::user::User;
use wasm_bindgen_futures::spawn_local;
use yew::html;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::api::user_api::api_register_user;

use super::main_app::Route;

pub enum LoginMessage {
    LoginButtonClicked,
    RegisterButtonClicked,
}

pub struct Login {}

impl Component for Login {
    type Message = LoginMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMessage::LoginButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::Workspace);

                //if !self.client.is_connected() {
                //    self.client.connect();
                //}
                //self.client.send_message_to_server(ClientMessage::Test);
            }
            LoginMessage::RegisterButtonClicked => {
                let user = User::new("name".to_string());
                spawn_local(async move {
                    let user = api_register_user(user).await.unwrap();
                    log::info!("return after calling api {user:?}");
                });
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let login_button_clicked = ctx.link().callback(|_| LoginMessage::LoginButtonClicked);
        let register_button_clicked = ctx.link().callback(|_| LoginMessage::RegisterButtonClicked);

        html!(
            <body>
                <div class="center">
                    <input id="username" style="display:block; width:100px; box-sizing: border-box" type="text" placeholder="username" />
                    <button onclick={login_button_clicked}> {"Login"} </button>
                    <button onclick={register_button_clicked}> {"Register"} </button>
                </div>
            </body>
        )
    }
}
