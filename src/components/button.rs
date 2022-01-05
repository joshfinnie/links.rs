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
    let color = match &ch as &str {
        "twitter" => "#1DA1F2",
        "github" => "#181717",
        "linkedin" => "#0077B5",
        "instagram" => "#E1306C",
        "twitch" => "#9146FF",
        _ => "#000000",
    };

    html! {
        <div>
            <a href={ props.url.to_owned() } target="_blank" class={css!("text-decoration: none;")}>
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
                        border-radius: .5rem;
                        border-color: ${bg};
                        background-color: ${bg};
                        color: white;
                    "#,
                    bg = color
                )}>
                    { ch }
                </button>
            </a>
        </div>
    }
}
