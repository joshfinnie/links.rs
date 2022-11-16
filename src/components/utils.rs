use stylist::yew::{styled_component, Global};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[styled_component(Container)]
pub fn container(props: &Props) -> Html {
    html! {
        <>
            <Global css={css!(
                r#"
                    html, body {
                        font-family: IBM Plex Sans,ui-sans-serif,system-ui,sans-serif;
                        padding: 0;
                        margin: 0;
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        min-height: 100vh;
                        flex-direction: column;
                        color: white;
                        background-color: #333;
                    }
                "#
            )} />
            <div class={css!(
                r#"
                    margin-left: auto;
                    margin-right: auto;
                    width: 250px;
                    z-index: 2;
                "#
            )}>
                { props.children.clone() }
            </div>
        </>
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
