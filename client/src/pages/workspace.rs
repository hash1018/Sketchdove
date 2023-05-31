use yew::{html, Component};

pub struct Workspace {}

impl Component for Workspace {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Workspace {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <body> { "Hello workspace" } </body>
        }
    }
}
