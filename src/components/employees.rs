use crate::components::employee_modal::{EmployeeModal, ModalMode};
use crate::db::{Employee, delete_employee, get_employees_server};
use dioxus::prelude::*;

#[component]
pub fn Employees() -> Element {
    let mut employees = use_resource(move || async move { get_employees_server().await });
    let mut modal_state = use_signal(|| None::<ModalMode>);
    let mut delete_confirm = use_signal(|| None::<Employee>);

    let handle_delete = move |id: i32| {
        spawn(async move {
            if let Ok(_) = delete_employee(id).await {
                delete_confirm.set(None);
                employees.restart();
            }
        });
    };

    rsx! {
        div {
            class: "container mx-auto px-4 py-8",

            // Header with Add Button
            div {
                class: "flex justify-between items-center mb-6",
                h1 {
                    class: "text-3xl font-bold text-gray-800",
                    "Employee Directory"
                }
                button {
                    class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 flex items-center gap-2",
                    onclick: move |_| modal_state.set(Some(ModalMode::Create)),
                    span { "+" }
                    span { "Add Employee" }
                }
            }

            // Table
            match employees() {
                None => rsx! {
                    div {
                        class: "flex justify-center items-center py-12",
                        p { class: "text-gray-600", "Loading employees..." }
                    }
                },
                Some(Err(err)) => rsx! {
                    div {
                        class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded",
                        "Error loading employees: {err}"
                    }
                },
                Some(Ok(employee_list)) => {

                    rsx! {
                        div {
                            class: "bg-white shadow-md rounded-lg overflow-hidden",
                            table {
                                class: "min-w-full divide-y divide-gray-200",
                                thead {
                                    class: "bg-gray-50",
                                    tr {
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "ID" }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "First Name" }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "Last Name" }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "Email" }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "Actions" }
                                    }
                                }
                                tbody {
                                    class: "bg-white divide-y divide-gray-200",
                                    for employee in employee_list.iter() {
                                        tr {
                                            key: "{employee.id}",
                                            class: "hover:bg-gray-50",
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{employee.id}" }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{employee.first_name}" }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{employee.last_name}" }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-600", "{employee.email}" }
                                            td {
                                                class: "px-6 py-4 whitespace-nowrap text-sm font-medium flex gap-2",
                                                button {
                                                    class: "text-blue-600 hover:text-blue-900",
                                                    onclick: {
                                                        let employee = employee.clone();
                                                        move |_| modal_state.set(Some(ModalMode::Edit(employee.clone())))
                                                    },
                                                    "✏️ Edit"
                                                }
                                                button {
                                                    class: "text-red-600 hover:text-red-900",
                                                    onclick: {
                                                        let emp = employee.clone();
                                                        move |_| delete_confirm.set(Some(emp.clone()))
                                                    },
                                                    "🗑️ Delete"
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if employee_list.is_empty() {
                                div {
                                    class: "text-center py-8 text-gray-500",
                                    "No employees found."
                                }
                            }
                        }
                    }
                }
            }
        }

        // Modal
        if let Some(mode) = modal_state() {
            EmployeeModal {
                mode: mode,
                on_close: move |_| modal_state.set(None),
                on_save: move |_| employees.restart(),
            }
        }

        // Delete Confirmation
        if let Some(employee) = delete_confirm() {
            div {
                class: "fixed inset-0 flex items-center justify-center z-50",
                style: "background-color: rgba(0, 0, 0, 0.4);",
                div {
                    class: "bg-white rounded-lg p-6 max-w-sm mx-4",
                    h3 { class: "text-lg font-bold mb-4", "Delete Employee?" }
                    p { class: "text-gray-600 mb-4", "Are you sure you want to delete {employee.first_name} {employee.last_name}?" }
                    p { class: "text-gray-500 text-sm mb-6", "This action cannot be undone." }
                    div {
                        class: "flex justify-end gap-3",
                        button {
                            class: "px-4 py-2 text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200",
                            onclick: move |_| delete_confirm.set(None),
                            "Cancel"
                        }
                        button {
                            class: "px-4 py-2 text-white bg-red-600 rounded-md hover:bg-red-700",
                            onclick: {
                                let id = employee.id;
                                move |_| handle_delete(id)
                            },
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}
