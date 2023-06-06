use std::rc::Rc;

use crate::api::user_api::api_login_user;
use crate::api::user_api::api_register_user;
use crate::LoginedUser;
use lib::user::User;
use wasm_bindgen_futures::spawn_local;
use yew::html;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use super::main_app::Route;

pub enum LoginMessage {
    LoginButtonClicked,
    RegisterButtonClicked,
    ContextChanged(Rc<LoginedUser>),
}

pub struct Login {
    logined_user: Rc<LoginedUser>,
    _listener: ContextHandle<Rc<LoginedUser>>,
}

impl Component for Login {
    type Message = LoginMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (logined_user, _listener) = ctx
            .link()
            .context::<Rc<LoginedUser>>(ctx.link().callback(LoginMessage::ContextChanged))
            .unwrap();
        Self {
            logined_user,
            _listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMessage::LoginButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                let user = User::new("name".to_string());
                let user_clone = user.clone();
                let logined_user = self.logined_user.clone();
                spawn_local(async move {
                    if let Ok(()) = api_login_user(&user).await {
                        logined_user.login(user);
                        navigator.push(&Route::Workspace);
                    }
                    log::info!("return after calling api login user {user_clone:?}");
                });
            }
            LoginMessage::RegisterButtonClicked => {
                let is_logined = self.logined_user.is_logined();
                log::info!("is_logined? {is_logined}");
                let user = User::new("name".to_string());
                spawn_local(async move {
                    api_register_user(&user).await.unwrap();
                    log::info!("return after calling api register user {user:?}");
                });
            }
            LoginMessage::ContextChanged(logined_user) => {
                self.logined_user = logined_user;
                return false;
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
