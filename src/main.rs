use yew_router::prelude::*;
use yew::prelude::*;

pub mod components;
use crate::components::{avatar::Avatar};

struct Container {}

#[derive(PartialEq, Properties)]
struct ContainerProps {
    children: Children,
}

impl Component for Container {
    type Message = ();
    type Properties = ContainerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container mx-auto my-10">
                <Avatar 
                    src="https://avatars.githubusercontent.com/u/125098"
                    alt="Josh Finnie's Avatar"
                />
                { ctx.props().children.clone() }
            </div>
        }
    }
}

struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let props = yew::props!(Container::Properties {
            children: Children::default(),
        });

        html! {
            <Container ..props>
                <h1 class="font-bold text-3xl text-center pb-3">
                    { "Home" }
                </h1>
                <p>{ "This is the home page!" }</p>
            </Container>
        }
    }
}

struct FourOhFour {}

impl Component for FourOhFour {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let props = yew::props!(Container::Properties {
            children: Children::default(),
        });

        html! {
            <Container ..props>
                <h1 class="font-bold text-3xl text-center pb-3">
                    { "404" }
                </h1>
                <p>{ "Something went wrong! We could not find the page you are looking for..." }</p>
            </Container>
        }
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <FourOhFour /> },
    }
}

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}