use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub byline: String,
}

#[styled_component(Header)]
pub fn header(props: &Props) -> Html {
    html! {
        <>
            <h1 class={css!(
                r#"
                    text-align: center;
                "#
            )}>{ props.name.to_owned() }</h1>
            <p>{ props.byline.to_owned() }</p>
        </>
    }
}
