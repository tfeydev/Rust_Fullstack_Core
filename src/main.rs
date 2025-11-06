use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        // document::Link { rel: "stylesheet", href: MAIN_CSS } 
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen bg-gray-100 text-gray-800",
            h1 {
                class: "text-4xl font-bold text-blue-500 mb-4",
                "Rust Fullstack Core"
            }
            p {
                class: "text-lg text-gray-600",
                "Clean Dioxus + TailwindCSS base ready."
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
        }

        Outlet::<Route> {}
    }
}
