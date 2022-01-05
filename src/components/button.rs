use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub channel: String,
    pub url: String,
}

#[styled_component(SocialButton)]
pub fn social_button(props: &Props) -> Html {
    let ch = props.channel.to_owned();
    html! {
        <div>
            <a href={ props.url.to_owned() } target="_blank">
                <button class={css!(
                    r#"
                        margin-top: .5rem;
                        margin-bottom: .5rem;
                        display: block;
                        width: 100%;
                        padding: 14px 28px;
                        font-size: 16px;
                        cursor: pointer;
                        text-align: center;
                        text-transform: capitalize;
                    "#
                )}>
                    { ch }
                </button>
            </a>
        </div>
    }
}
