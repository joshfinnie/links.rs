use std::collections::HashMap;

use log;
use serde_derive::Deserialize;
use toml;
use wasm_logger;
use yew::prelude::*;
use yew_router::prelude::*;

pub mod components;
use crate::components::{
    avatar::Avatar,
    button::SocialButton,
    header::Header,
    utils::{Container, FourOhFour},
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

#[function_component(Home)]
pub fn home() -> Html {
    let props = yew::props!(Container::Properties {
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
                byline={ conf.bio.byline }
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
                        />
                    }
                })
            }
            <p>{ conf.bio.footer }</p>
        </Container>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <FourOhFour /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("started");
    yew::start_app::<App>();
}
