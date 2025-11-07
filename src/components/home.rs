use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "text-center",
            h1 {
                class: "text-lg text-gray-600",
                "Clean Dioxus + TailwindCSS base ready."
            }
        }
    }
}
