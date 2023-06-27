use web_sys::HtmlInputElement;
use yew::{html, Component, Properties};
use yew::{Callback, NodeRef};

pub enum LoginMessage {
    JoinButtonClicked,
    CreateRoomButtonClicked,
}

pub enum LoginNotifyMessage {
    EnterRoom(String, Option<String>),
}

#[derive(Clone, PartialEq, Properties)]
pub struct LoginProps {
    pub hide_create_button: bool,
    pub handler: Callback<LoginNotifyMessage>,
}

pub struct Login {
    user_name_ref: NodeRef,
    room_id_ref: NodeRef,
}

impl Component for Login {
    type Message = LoginMessage;
    type Properties = LoginProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            user_name_ref: NodeRef::default(),
            room_id_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let user_name = self
            .user_name_ref
            .cast::<HtmlInputElement>()
            .unwrap()
            .value();

        let room_id = self
            .room_id_ref
            .cast::<HtmlInputElement>()
            .map(|room_id| room_id.value());

        log::info!("user_name {user_name}, room_id {room_id:?}");

        if let Some(room_id) = room_id.as_ref() {
            if user_name.is_empty() || room_id.is_empty() {
                return false;
            }
        } else if user_name.is_empty() {
            return false;
        }

        match msg {
            LoginMessage::JoinButtonClicked => {
                ctx.props()
                    .handler
                    .emit(LoginNotifyMessage::EnterRoom(user_name, room_id));
            }
            LoginMessage::CreateRoomButtonClicked => {
                ctx.props()
                    .handler
                    .emit(LoginNotifyMessage::EnterRoom(user_name, room_id));
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let join_button_clicked = ctx.link().callback(|_| LoginMessage::JoinButtonClicked);
        let create_button_clicked = ctx
            .link()
            .callback(|_| LoginMessage::CreateRoomButtonClicked);

        let hide_create_button = ctx.props().hide_create_button;

        html! {
           <div>
               <input id="username" ref={&self.user_name_ref} type="text" placeholder="username" />
               if !hide_create_button {
                    <input id="room_id" ref={&self.room_id_ref} type="text" placeholder="room id" />
               }

               <button onclick={join_button_clicked}> {"Join"} </button>
               if !hide_create_button {
                    <button onclick={create_button_clicked}> {"Create Room"} </button>
               }
           </div>
        }
    }
}
