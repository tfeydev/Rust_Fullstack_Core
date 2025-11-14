use dioxus::prelude::*;
use dioxus_router::components::Link;
use crate::server::state::AuthState;

#[component]
pub fn Header() -> Element {
    let mut auth = use_context::<AuthState>();

    rsx! {
        header {
            class: "fixed top-0 left-0 right-0 bg-blue-600 text-white p-4 shadow-md z-50",
            div {
                class: "container mx-auto flex justify-between items-center",
                h1 { class: "text-2xl font-bold", "Rust Fullstack Core" }
                nav {
                    class: "flex gap-4",

                    // always visible
                    Link { to: "/", class: "hover:text-blue-200", "Home" }
                    Link { to: "/login", class: "hover:text-blue-200", "Login" }

                    // only visible when logged in
                    if (auth.logged_in)() {
                        Link { to: "/employees", class: "hover:text-blue-200", "Employees" }
                        Link { to: "/users", class: "hover:text-gray-200", "Users" }
                        button {
                            class: "ml-4 bg-red-600 text-white px-3 py-1 rounded",
                            onclick: move |_| auth.logged_in.set(false),
                            "Logout"
                        }
                    }
                }
            }
        }
    }
}
