#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::{Routable, Router};

mod components;
// mod db;

// Import the components we want to use
use crate::components::{footer::Footer, header::Header, home::Home};

// Define the application routes
#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[route("/")]
    Home {},
}

// Main entry point for the application
fn main() {
    dioxus::launch(App);
}

// The root App component
#[component]
fn App() -> Element {
    rsx! {
        // Link to the stylesheet and favicon
        document::Link { rel: "stylesheet", href: asset!("assets/tailwind.css") }
        document::Link { rel: "icon", href: asset!("assets/favicon.ico") }

        // The main layout with a fixed header and footer
        div {
            class: "bg-gray-100 text-gray-800",

            Header {}

            main {
                // Padding top/bottom to avoid overlap with fixed header/footer
                // Min-height and flex to center content on short pages
                class: "min-h-screen flex flex-col items-center justify-center pt-20 pb-20",
                Router::<Route> {}
            }

            Footer {}
        }
    }
}
