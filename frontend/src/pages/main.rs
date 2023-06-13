use yew::{html, Component};

pub struct Main {}

impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! { "main" }
    }
}
