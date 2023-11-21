use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub channel: String,
    pub url: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub title: Option<String>,
}

struct Channel {
    color: String,
    title: String,
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
        "mastodon" => Channel {
            color: String::from("#6364FF"),
            title: if props.title.is_some() { props.title.clone().unwrap() } else {String::from(uppercase_first_letter(&ch))},
            icon: String::from("fa-brands fa-mastodon"),
        },
        "twitter" => Channel {
            color: String::from("#1DA1F2"),
            title: if props.title.is_some() { props.title.clone().unwrap() } else {String::from(uppercase_first_letter(&ch))},
            icon: String::from("fa fa-twitter"),
        },
        "github" => Channel {
            color: String::from("#181717"),
            title: if props.title.is_some() { props.title.clone().unwrap() } else {String::from(uppercase_first_letter(&ch))},
            icon: String::from("fa-brands fa-github"),
        },
        "linkedin" => Channel {
            color: String::from("#0077B5"),
            title: if props.title.is_some() { props.title.clone().unwrap() } else {String::from(uppercase_first_letter(&ch))},
            icon: String::from("fa-brands fa-linkedin-in"),
        },
        "twitch" => Channel {
            color: String::from("#9146FF"),
            title: if props.title.is_some() { props.title.clone().unwrap() } else {String::from(uppercase_first_letter(&ch))},
            icon: String::from("fa-brands fa-twitch"),
        },
        "instagram" => Channel {
            color: String::from("#E1306C"),
            title: if props.title.is_some() { props.title.clone().unwrap() } else {String::from(uppercase_first_letter(&ch))},
            icon: String::from("fa-brands fa-instagram"),
        },
        "email" => Channel {
            color: String::from("#000000"),
            title: if props.title.is_some() { props.title.clone().unwrap() } else {String::from(uppercase_first_letter(&ch))},
            icon: String::from("fa-regular fa-envelope"),
        },
        "youtube" => Channel {
            color: String::from("#ff0000"),
            title: if props.title.is_some() { props.title.clone().unwrap() } else {String::from(uppercase_first_letter(&ch))},
            icon: String::from("fa-brands fa-youtube"),
        },
        "threads" => Channel {
            color: String::from("#1c1e21"),
            title: if props.title.is_some() { props.title.clone().unwrap() } else {String::from(uppercase_first_letter(&ch))},
            icon: String::from("fa-brands fa-threads"),
        },
        _ => Channel {
            color: match &props.color {
                Some(s) => String::from(s),
                None => String::from("#000000"),
            },
            title: if props.title.is_some() { props.title.clone().unwrap() } else {String::from(uppercase_first_letter(&ch))},
            icon: match &props.icon {
                Some(s) => String::from(s),
                None => String::from("default"),
            },
        },
    };

    let s = use_style!(r#"
        padding-left: 1rem;
        justify-self: start;
        text-size: 2rem;
    "#);

    html! {
        <div>
            <a href={ props.url.to_owned() } rel="me" target="_blank" class={css!(
                r#"
                    margin-top: .5rem;
                    margin-bottom: .5rem;
                    width: 100%;
                    padding: 10px 0;
                    font-size: 16px;
                    cursor: pointer;
                    border-radius: 1rem;
                    border: none;
                    background-color: ${bg};
                    color: white;
                    text-decoration: none;
                    display: grid;
                    grid-template-columns: auto 1fr;
                    justify-content: center;
                    align-items: center;
                    gap: 2rem;
                    &:hover {
                        background-color: gray;
                        color: black;
                    }
                "#,
                bg = channel.color.to_owned(),
            )}>
                <i class={classes!("fa-fw", "fa-2x", channel.icon, s)} aria-hidden="true"></i>
                <span>{ channel.title }</span>
            </a>
        </div>
    }
}
