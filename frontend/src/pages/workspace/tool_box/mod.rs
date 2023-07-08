use crate::{base::DrawModeType, pages::workspace::workspace::ChildRequestType};
use yew::{html, Callback, Component, Properties};

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
        let select_button_clicked = ctx
            .props()
            .handler
            .reform(|_| ChildRequestType::ChangeMode(DrawModeType::SelectMode));

        let line_button_clicked = ctx
            .props()
            .handler
            .reform(|_| ChildRequestType::ChangeMode(DrawModeType::LineMode));

        let current_mode = ctx.props().current_mode;

        html! {
            <div style="height: 100%; overflow: hidden;">
                <button id="select_button" class={ select_button_class(current_mode) }  onclick={select_button_clicked}></button>
                <button id="line_button" class={ line_button_class(current_mode) } onclick={line_button_clicked}></button>
            </div>
        }
    }
}

fn select_button_class(current_mode: DrawModeType) -> String {
    format!(
        "{0} tool_box_select_button",
        render_tool_button(current_mode, DrawModeType::SelectMode)
    )
}

fn line_button_class(current_mode: DrawModeType) -> String {
    format!(
        "{0} tool_box_line_button",
        render_tool_button(current_mode, DrawModeType::LineMode)
    )
}

fn render_tool_button(current_mode: DrawModeType, target_mode: DrawModeType) -> String {
    if current_mode == target_mode {
        "tool_box_button_selected".to_string()
    } else {
        "tool_box_button".to_string()
    }
}
