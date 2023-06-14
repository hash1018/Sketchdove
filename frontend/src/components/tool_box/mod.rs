use yew::{html, Callback, Component, Properties};

use crate::{algorithm::draw_mode::DrawModeType, pages::workspace::ChildRequestType};

#[derive(Clone, PartialEq, Properties)]
pub struct ToolBoxProps {
    pub handler: Callback<ChildRequestType>,
    pub current_mode: DrawModeType,
}
pub struct ToolBox {}

impl Component for ToolBox {
    type Message = ();
    type Properties = ToolBoxProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let line_button_clicked = ctx
            .props()
            .handler
            .reform(|_| ChildRequestType::ChangeMode(DrawModeType::LineMode));

        let text = format!("{0:?}", ctx.props().current_mode);
        html! {
            <div style="height: 100%; overflow: hidden;">
                <button onclick={line_button_clicked}> {"Line"} </button>
                <font color="#FFFFFF"> {text} </font>
            </div>
        }
    }
}
