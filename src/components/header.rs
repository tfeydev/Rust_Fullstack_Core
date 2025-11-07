use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "fixed top-0 left-0 right-0 bg-blue-600 text-white p-4 shadow-md z-50",
            h1 { class: "text-2xl font-bold", "Rust Fullstack Core" }
        }
    }
}
