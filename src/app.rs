#![allow(non_snake_case)]
use crate::components::{footer::Footer, header::Header, home::Home};
use dioxus::prelude::*;
use dioxus_router::{Routable, Router};

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Home {},
}

pub fn App() -> Element {
    rsx! {
    div {
        class: "bg-gray-100 text-gray-800",

        Header {}

        main {
            class: "min-h-screen flex flex-col items-center justify-center pt-20 pb-20",
            Router::<Route> {}
        }

        Footer {}
    }    }
}
