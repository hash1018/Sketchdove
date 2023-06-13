use yew::{html, Component};
use yew_router::scope_ext::RouterScopeExt;

use super::app::{set_user_name, Route};

pub enum MainMessage {
    JoinButtonClicked,
    CreateRoomButtonClicked,
}

pub struct Main {}

impl Component for Main {
    type Message = MainMessage;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        set_user_name(None);
        Self {}
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MainMessage::JoinButtonClicked => {
                let id = "join".to_string();
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::Workspace { id });
            }
            MainMessage::CreateRoomButtonClicked => {
                set_user_name(Some("hello".to_string()));

                let id = "random".to_string();
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::Workspace { id });
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let join_button_clicked = ctx.link().callback(|_| MainMessage::JoinButtonClicked);
        let create_button_clicked = ctx
            .link()
            .callback(|_| MainMessage::CreateRoomButtonClicked);

        html! {
           <div>
               <input id="username" type="text" placeholder="username" />
               <input id="room_id" type="text" placeholder="room id" />
               <button onclick={join_button_clicked}> {"Join"} </button>
               <button onclick={create_button_clicked}> {"Create Room"} </button>
           </div>
        }
    }
}
