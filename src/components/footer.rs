use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "fixed bottom-0 left-0 right-0 bg-gray-500 text-gray-200 py-4 text-center border-t border-gray-700 z-50",
            "© 2025 Thorsten Fey"
        }
    }
}
