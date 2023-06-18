use yew::{html, Callback, Component, Properties};

use crate::pages::workspace::ChildRequestType;

#[derive(Clone, PartialEq, Properties)]
pub struct TitleBarProps {
    pub handler: Callback<ChildRequestType>,
    pub show_chat: bool,
}

pub enum TitleBarMessage {}

pub struct TitleBar {}

impl Component for TitleBar {
    type Message = TitleBarMessage;
    type Properties = TitleBarProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let leave_button_clicked = ctx.props().handler.reform(|_| ChildRequestType::Leave);

        let show_chat = ctx.props().show_chat;
        let chat_button_clicked = ctx
            .props()
            .handler
            .reform(move |_| ChildRequestType::ShowChat(!show_chat));

        html!(
            <div style="height: 100%; overflow: hidden;">
                <button class="leave_button" onclick={leave_button_clicked}></button>
                <button class={chat_button_css(show_chat)} onclick={chat_button_clicked}></button>
            </div>
        )
    }
}

fn chat_button_css(show_chat: bool) -> String {
    if show_chat {
        String::from("chat_button_selected")
    } else {
        String::from("chat_button")
    }
}
