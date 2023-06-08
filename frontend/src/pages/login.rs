use crate::api::user_api::api_check_login_valid;
use crate::api::user_api::api_login_user;
use crate::api::user_api::api_register_user;
use lib::user::User;
use wasm_bindgen_futures::spawn_local;
use yew::html;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use super::main_app::Route;

pub enum LoginMessage {
    LoginButtonClicked,
    RegisterButtonClicked,
}

fn check_login_valid(ctx: &Context<Login>) {
    let navigator = ctx.link().navigator().unwrap();
    spawn_local(async move {
        if let Ok(()) = api_check_login_valid().await {
            navigator.replace(&Route::Workspace);
        }
    });
}

pub struct Login {}

impl Component for Login {
    type Message = LoginMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        check_login_valid(ctx);
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMessage::LoginButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                let user = User::new("name".to_string());
                spawn_local(async move {
                    if let Ok(()) = api_login_user(&user).await {
                        navigator.replace(&Route::Workspace);
                    }
                });
            }
            LoginMessage::RegisterButtonClicked => {
                let user = User::new("name".to_string());
                spawn_local(async move {
                    api_register_user(&user).await.unwrap();
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
