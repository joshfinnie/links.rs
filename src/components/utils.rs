use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[styled_component(Container)]
pub fn container(props: &Props) -> Html {
    html! {
        <div class={css!(
            r#"
                margin-top: 2.5rem;
                margin-left: auto;
                margin-right: auto;
                width: 250px;
                z-index: 2;
            "#
        )}>
            { props.children.clone() }
        </div>
    }
}

#[function_component(FourOhFour)]
pub fn four_oh_four() -> HTML {
    let props = yew::props!(Container::Properties {
        children: Children::default(),
    });

    html! {
        <Container ..props>
            <h1>
                { "404" }
            </h1>
            <p>{ "Something went wrong! We could not find the page you are looking for..." }</p>
        </Container>
    }
}
