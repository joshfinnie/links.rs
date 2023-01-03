use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub byline: String,
}

#[styled_component(Header)]
pub fn header(props: &Props) -> Html {
    let byline: Html = Html::from_html_unchecked(props.byline.to_owned().into());
    html! {
        <>
            <h1 class={css!(
                r#"
                    text-align: center;
                "#
            )}>{ props.name.to_owned() }</h1>
            { byline }
        </>
    }
}
