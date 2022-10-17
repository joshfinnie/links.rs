use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub channel: String,
    pub url: String,
    pub color: Option<String>,
    pub icon: Option<String>,
}

struct Channel {
    color: String,
    display: String,
    icon: String,
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[styled_component(SocialButton)]
pub fn social_button(props: &Props) -> Html {

    let ch = props.channel.to_owned();
    let channel = match &ch as &str {
        "twitter" => Channel {
            color: String::from("#1DA1F2"),
            display: String::from(uppercase_first_letter(&ch)),
            icon: String::from("fa fa-twitter"),
        },
        "github" => Channel {
            color: String::from("#181717"),
            display: String::from(uppercase_first_letter(&ch)),
            icon: String::from("fa fa-github"),
        },
        "linkedin" => Channel {
            color: String::from("#0077B5"),
            display: String::from(uppercase_first_letter(&ch)),
            icon: String::from("fa fa-linkedin"),
        },
        "twitch" => Channel {
            color: String::from("#9146FF"),
            display: String::from(uppercase_first_letter(&ch)),
            icon: String::from("fa fa-twitch"),
        },
        "instagram" => Channel {
            color: String::from("#E1306C"),
            display: String::from(uppercase_first_letter(&ch)),
            icon: String::from("fa fa-instagram"),
        },
        "email" => Channel {
            color: String::from("#000000"),
            display: String::from(uppercase_first_letter(&ch)),
            icon: String::from("fa fa-envelope"),
        },
        "youtube" => Channel {
            color: String::from("#ff0000"),
            display: String::from(uppercase_first_letter(&ch)),
            icon: String::from("fa fa-youtube"),
        },
        _ => Channel {
            color: match &props.color {
                Some(s) => String::from(s),
                None => String::from("#000000"),
            },
            display: String::from(uppercase_first_letter(&ch)),
            icon: match &props.icon {
                Some(s) => String::from(s),
                None => String::from("default"),
            },
        }
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
                        border-radius: .5rem;
                        border: none;
                        background-color: ${bg};
                        color: white;
                    "#,
                    bg = channel.color.to_owned()
                )}>
                    <i class={channel.icon} aria-hidden="true"></i>
                    <span class={css!(
                        r#"
                            padding-left: 5px;
                        "#,
                    )}>{ channel.display }</span>
                </button>
            </a>
        </div>
    }
}
