use yew::{html, Component, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct WorkspaceProps {
    pub id: String,
}

pub struct Workspace {}

impl Component for Workspace {
    type Message = ();
    type Properties = WorkspaceProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let id = ctx.props().id.clone();
        html!({ format!("workspace {id}") })
    }
}
