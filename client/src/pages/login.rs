use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

pub enum Message {
    LoginButtonClicked,
}

pub struct Login {}

impl Component for Login {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::LoginButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::Workspace);
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
