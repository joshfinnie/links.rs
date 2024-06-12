use std::collections::HashMap;

use log;
use serde_derive::Deserialize;
use stylist::yew::styled_component;
use toml;
use wasm_logger;
use yew::prelude::*;

pub mod components;
use crate::components::{
    avatar::Avatar,
    button::SocialButton,
    header::Header,
    utils::{Container, Props},
};

const DATA: &str = include_str!("../.config.toml");

#[derive(Deserialize)]
struct Bio {
    name: String,
    avatar: String,
    avatar_alt: String,
    byline: String,
    footer: String,
}

#[derive(Clone, Debug, Deserialize)]
struct SocialEntity {
    url: String,
    color: Option<String>,
    icon: Option<String>,
    title: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    button_order: String,
}

#[derive(Deserialize)]
struct Config {
    bio: Bio,
    social: HashMap<String, SocialEntity>,
    settings: Settings,
}

impl Default for Config {
    fn default() -> Self {
        toml::from_str(DATA).unwrap()
    }
}

#[styled_component(Home)]
pub fn home() -> Html {
    let props = yew::props!(Props {
        children: Children::default(),
    });

    let conf = Config::default();
    let button_order = conf.settings.button_order.split(",").collect::<Vec<&str>>();

    html! {
        <Container ..props>
            <Avatar
                src={ conf.bio.avatar }
                alt={ conf.bio.avatar_alt }
            />
            <Header
                name={ conf.bio.name }
                byline={ markdown::to_html(&conf.bio.byline) }
            />
            {
                for button_order.iter().map(|&b| {
                    let data = conf.social.get(&String::from(b)).unwrap();
                    html! {
                        <SocialButton
                            channel={ String::from(b).clone() }
                            url={ data.url.clone() }
                            color={ data.color.clone() }
                            icon={ data.icon.clone() }
                            title={ data.title.clone() }
                        />
                    }
                })
            }
            <p>{ conf.bio.footer }</p>
            { html!{
                <p>
                    {"This was built using Links.rs. You can find the source code on "}
                    <a class={css!("color: white;")} href="https://github.com/joshfinnie/links.rs">
                        {"Github"}
                    </a>
                    {". Give it a ⭐!"}
                </p>
            }}
        </Container>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Home />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("started");
    yew::Renderer::<App>::new().render();
}
