#![allow(non_snake_case)]
use crate::components::{
    employees::Employees, 
    footer::Footer, 
    header::Header, 
    home::Home, 
    users::Users,
    login::Login,
};
use dioxus::prelude::*;
use dioxus_router::{Routable, Router};
use crate::server::state::AuthState;

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    
    Home {},
    
    #[route("/employees")]
    Employees {},
    
    #[route("/users")]
    Users {},
    
    #[route("/login")]
    Login {},

}

#[component]
pub fn App() -> Element {
    let logged_in = use_signal(|| false);
    
    use_context_provider(|| AuthState { logged_in });
    
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("assets/tailwind.css") }
        document::Link { rel: "icon", href: asset!("assets/favicon.ico") }

        Router::<Route> {}
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        div {
            class: "bg-gray-100 text-gray-800 min-h-screen flex flex-col",

            Header {}

            main {
                class: "flex-1 flex flex-col items-center justify-center pt-20 pb-20",
                Outlet::<Route> {}
            }

            Footer {}
        }
    }
}
