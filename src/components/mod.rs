use dioxus::prelude::*;
use dioxus_router::{Routable, Router};
use crate::components::home::Home;

pub mod home;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
