use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub src: String,
    pub alt: String,
}

#[styled_component(Avatar)]
pub fn avatar(props: &Props) -> Html {
    html! {
        <img
            class={css!(
                r#"
                    color: white;
                    border-radius: 50%;
                    border: none;
                    display: block;
                    margin-left: auto;
                    margin-right: auto;
                    width: 50%;
                "#
            )}
            src={props.src.to_owned()}
            alt={props.alt.to_owned()}
        />
    }
}
