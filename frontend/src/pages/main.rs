use yew::{html, Component};
use yew_router::scope_ext::RouterScopeExt;

use crate::components::login::{Login, LoginNotifyMessage};

use super::app::{set_user_name, Route};

pub enum MainMessage {
    HandleLoginNotifyMessage(LoginNotifyMessage),
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
            MainMessage::HandleLoginNotifyMessage(msg) => match msg {
                LoginNotifyMessage::EnterRoom(name, room_id) => {
                    set_user_name(Some(name));
                    let navigator = ctx.link().navigator().unwrap();
                    navigator.push(&Route::Workspace {
                        id: room_id.unwrap(),
                    });
                }
            },
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let handler = ctx.link().callback(MainMessage::HandleLoginNotifyMessage);
        html! {
           <div>
               <Login {handler} hide_create_button = {false} />
           </div>
        }
    }
}
