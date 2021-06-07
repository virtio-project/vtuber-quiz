use std::fmt;
use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod components;

#[derive(Switch, Debug, Clone)]
enum AppRoute {
    #[to = "/accounts/login"]
    Login,
    #[to = "/accounts/register"]
    Register,
    #[to = "/"]
    Home,
}

enum Msg {
    AddOne,
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Router<AppRoute> render=Router::render(switch) />
            </div>
        }
    }
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => {
            html! { <pages::Home /> }
        }
        AppRoute::Login => {
            html! { <pages::accounts::Login /> }
        }
        AppRoute::Register => {
            html! { <pages::accounts::Register /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::start_app::<Model>();
}
