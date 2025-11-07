use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "text-center",
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
