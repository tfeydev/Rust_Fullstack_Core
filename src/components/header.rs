use dioxus::prelude::*;
use dioxus_router::components::Link;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "fixed top-0 left-0 right-0 bg-blue-600 text-white p-4 shadow-md z-50",
            div {
                class: "container mx-auto flex justify-between items-center",
                h1 {
                    class: "text-2xl font-bold",
                    "Rust Fullstack Core"
                }
                nav {
                    class: "flex gap-4",
                    Link {
                        to: "/",
                        class: "hover:text-blue-200",
                        "Home"
                    }
                    Link {
                        to: "/employees",
                        class: "hover:text-blue-200",
                        "Employees"
                    }
                }
            }
        }
    }
}
