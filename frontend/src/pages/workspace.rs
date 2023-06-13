use yew::{html, Component, Properties};

use crate::pages::app::user_name;

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
        let user_name = user_name();
        let id = ctx.props().id.clone();
        if let Some(user_name) = user_name {
            html!({ format!("workspace id:{id}, user_name:{user_name}") })
        } else {
            html!({ format!("workspace id:{id} user name is none") })
        }
    }
}
