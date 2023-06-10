use yew::{html, Callback, Component, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct TitleBarProps {
    pub handler: Callback<String>,
}

pub struct TitleBar {}

impl Component for TitleBar {
    type Message = ();
    type Properties = TitleBarProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let logout_button_clicked = ctx.props().handler.reform(|_| "logout".to_string());
        let register_button_clicked = ctx.props().handler.reform(|_| "register".to_string());

        html!(
            <div style="height: 100%; overflow: hidden;">
                <input id="username" style="display:block; width:100px; box-sizing: border-box" type="text" placeholder="username" />
                <button onclick={logout_button_clicked}> {"Logout"} </button>
                <button onclick={register_button_clicked}> {"Register"} </button>
            </div>
        )
    }
}
