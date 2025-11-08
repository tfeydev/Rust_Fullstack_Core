use crate::db::{Employee, create_employee, update_employee};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ModalMode {
    Create,
    Edit(Employee),
}

#[component]
pub fn EmployeeModal(
    mode: ModalMode,
    on_close: EventHandler<()>,
    on_save: EventHandler<()>,
) -> Element {
    let mut first_name = use_signal(|| match &mode {
        ModalMode::Create => String::new(),
        ModalMode::Edit(emp) => emp.first_name.clone().unwrap_or_default(),
    });

    let mut last_name = use_signal(|| match &mode {
        ModalMode::Create => String::new(),
        ModalMode::Edit(emp) => emp.last_name.clone().unwrap_or_default(),
    });

    let mut email = use_signal(|| match &mode {
        ModalMode::Create => String::new(),
        ModalMode::Edit(emp) => emp.email.clone().unwrap_or_default(),
    });

    let mut is_saving = use_signal(|| false);
    let mut error_message = use_signal(|| None::<String>);

    let mode_for_save = mode.clone();
    let handle_save = move |_| {
        let mode = mode_for_save.clone();
        spawn(async move {
            is_saving.set(true);
            error_message.set(None);

            let result = match &mode {
                ModalMode::Create => create_employee(first_name(), last_name(), email()).await,
                ModalMode::Edit(emp) => {
                    update_employee(emp.id, first_name(), last_name(), email()).await
                }
            };

            match result {
                Ok(_) => {
                    on_save.call(());
                    on_close.call(());
                }
                Err(e) => {
                    error_message.set(Some(format!("Error: {}", e)));
                }
            }

            is_saving.set(false);
        });
    };

    let title = match &mode {
        ModalMode::Create => "Add New Employee",
        ModalMode::Edit(_) => "Edit Employee",
    };

    let button_text = match &mode {
        ModalMode::Create => "Create",
        ModalMode::Edit(_) => "Update",
    };

    rsx! {
        div {
            class: "fixed inset-0 flex items-center justify-center z-50",
            style: "background-color: rgba(0, 0, 0, 0.4);",
            onclick: move |_| on_close.call(()),

            div {
                class: "bg-white rounded-lg shadow-xl max-w-md w-full mx-4",
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "flex justify-between items-center p-6 border-b",
                    h2 {
                        class: "text-xl font-bold text-gray-800",
                        "{title}"
                    }
                    button {
                        class: "text-gray-400 hover:text-gray-600",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                // Body
                div {
                    class: "p-6",

                    if let Some(error) = error_message() {
                        div {
                            class: "mb-4 p-3 bg-red-100 border border-red-400 text-red-700 rounded",
                            "{error}"
                        }
                    }

                    // First Name
                    div {
                        class: "mb-4",
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-2",
                            "First Name"
                        }
                        input {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "text",
                            value: "{first_name}",
                            oninput: move |e| first_name.set(e.value()),
                            placeholder: "Enter first name",
                        }
                    }

                    // Last Name
                    div {
                        class: "mb-4",
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-2",
                            "Last Name"
                        }
                        input {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "text",
                            value: "{last_name}",
                            oninput: move |e| last_name.set(e.value()),
                            placeholder: "Enter last name",
                        }
                    }

                    // Email
                    div {
                        class: "mb-4",
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-2",
                            "Email"
                        }
                        input {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "email",
                            value: "{email}",
                            oninput: move |e| email.set(e.value()),
                            placeholder: "Enter email address",
                        }
                    }
                }

                // Footer
                div {
                    class: "flex justify-end gap-3 p-6 border-t",
                    button {
                        class: "px-4 py-2 text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200",
                        onclick: move |_| on_close.call(()),
                        disabled: is_saving(),
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50",
                        onclick: handle_save,
                        disabled: is_saving(),
                        if is_saving() {
                            "Saving..."
                        } else {
                            "{button_text}"
                        }
                    }
                }
            }
        }
    }
}
