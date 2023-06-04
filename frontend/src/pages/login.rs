use yew::html;
use yew::prelude::*;

pub enum LoginMessage {
    LoginButtonClicked,
}

pub struct Login {}

impl Component for Login {
    type Message = LoginMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMessage::LoginButtonClicked => {
                //let navigator = ctx.link().navigator().unwrap();
                //navigator.push(&Route::Workspace);

                //if !self.client.is_connected() {
                //    self.client.connect();
                //}
                //self.client.send_message_to_server(ClientMessage::Test);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let login_button_clicked = ctx.link().callback(|_| LoginMessage::LoginButtonClicked);

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
