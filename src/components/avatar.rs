use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub src: String,
    pub alt: String,
}

pub struct Avatar {}

impl Component for Avatar {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <img
                class="rounded-full max-h-20 max-w-20"
                src={ctx.props().src.to_owned()}
                alt={ctx.props().alt.to_owned()}
            />
        }
    }
}