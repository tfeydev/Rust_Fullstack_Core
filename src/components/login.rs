use dioxus::prelude::*;

use crate::server::state::AuthState;

#[component]
pub fn Login() -> Element {
    // Signals for form fields
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    
    let _auth = use_context::<AuthState>();

    rsx! {
        div { 
            class: "bg-gray-100 flex justify-center pt-32 pb-20 min-h-[400px]",

            // Login card
            div {
                class: "bg-white shadow-lg rounded-lg p-8 w-full max-w-md",

                h2 {
                    class: "text-2xl font-bold text-center mb-6",
                    "Welcome Back"
                }

                p {
                    class: "text-gray-600 text-center mb-6",
                    "Please sign in to continue."
                }

                // Email field
                div { class: "mb-4",
                    label { class: "block mb-1 font-medium text-gray-700", "Email" }
                    input {
                        class: "w-full px-4 py-2 border rounded focus:ring-2 focus:ring-blue-400 focus:outline-none",
                        r#type: "email",
                        value: "{email()}",
                        placeholder: "you@example.com",
                        oninput: move |e| email.set(e.value().to_string()),
                    }
                }

                // Password field
                div { class: "mb-6",
                    label { class: "block mb-1 font-medium text-gray-700", "Password" }
                    input {
                        class: "w-full px-4 py-2 border rounded focus:ring-2 focus:ring-blue-400 focus:outline-none",
                        r#type: "password",
                        value: "{password()}",
                        placeholder: "••••••••",
                        oninput: move |e| password.set(e.value().to_string()),
                    }
                }

                // Login button
                button {
                    class: "w-full bg-blue-600 text-white py-2 rounded font-semibold hover:bg-blue-700 transition-colors",
                    onclick: move |_| {
                        println!("Login attempt: {} / {}", email(), password());
                    },
                    "Login"
                }

                // optional forgot password
                div {
                    class: "text-center mt-4",
                    a {
                        class: "text-blue-600 hover:underline cursor-pointer",
                        "Forgot password?"
                    }
                }
            }
        }
    }
}
