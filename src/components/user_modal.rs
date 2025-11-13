use crate::db::{User, create_user, update_user};
use dioxus::prelude::*;

#[derive(Clone)]
pub enum UserModalMode {
    Create,
    Edit(User),
}

#[component]
pub fn UserModal(
    mode: UserModalMode,
    on_close: EventHandler<()>,
    on_save: EventHandler<()>,
) -> Element {
    let email = use_signal(|| match &mode {
        UserModalMode::Create => "".to_string(),
        UserModalMode::Edit(u) => u.email.clone(),
    });

    let role = use_signal(|| match &mode {
        UserModalMode::Create => "ROLE_EMPLOYEE".to_string(),
        UserModalMode::Edit(u) => u.role_name.clone(),
    });

    rsx! {
        div {
            class: "fixed inset-0 flex items-center justify-center bg-black/30 z-50",

            div {
                class: "bg-white p-6 rounded-lg w-full max-w-md",

                h3 {
                    class: "text-xl font-bold mb-4",
                    match mode {
                        UserModalMode::Create => "Create User",
                        UserModalMode::Edit(_) => "Edit User",
                    }
                }

                // EMAIL
                div { class: "mb-4",
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Email" }
                    input {
                        class: "w-full px-3 py-2 border rounded",
                        value: "{email()}",
                        oninput: move |e| email.set(e.value())
                    }
                }

                // ROLE
                div { class: "mb-4",
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Role" }
                    select {
                        class: "w-full px-3 py-2 border rounded",
                        value: "{role()}",
                        oninput: move |e| role.set(e.value()),

                        option { value: "ROLE_ADMIN", "Admin" }
                        option { value: "ROLE_MANAGER", "Manager" }
                        option { value: "ROLE_HR", "HR" }
                        option { value: "ROLE_IT", "IT" }
                        option { value: "ROLE_EMPLOYEE", "Employee" }
                    }
                }

                // BUTTONS
                div { class: "flex justify-end gap-3",

                    button {
                        class: "px-4 py-2 bg-gray-200 text-gray-700 rounded",
                        onclick: move |_| on_close(()),
                        "Cancel"
                    }

                    button {
                        class: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700",
                        onclick: move |_| {
                            spawn({
                                let email = email();
                                let role = role();
                                let on_close = on_close.clone();
                                let on_save = on_save.clone();

                                async move {
                                    match &mode {
                                        UserModalMode::Create => {
                                            let _ = create_user(email, role).await;
                                        }
                                        UserModalMode::Edit(u) => {
                                            let _ = update_user(u.id, email, role).await;
                                        }
                                    }

                                    on_close(());
                                    on_save(());
                                }
                            });
                        },
                        "Save"
                    }
                }
            }
        }
    }
}
