use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

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
