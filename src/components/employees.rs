use crate::db::{Employee, get_employees_server};
use dioxus::prelude::*;

#[component]
pub fn Employees() -> Element {
    // Call server function to get employees
    let employees = use_resource(move || async move { get_employees_server().await });

    rsx! {
        div {
            class: "container mx-auto px-4 py-8",

            h1 {
                class: "text-3xl font-bold text-gray-800 mb-6",
                "Employee Directory"
            }

            // Loading, Error, or Data
            match employees() {
                None => rsx! {
                    div {
                        class: "flex justify-center items-center py-12",
                        p {
                            class: "text-gray-600",
                            "Loading employees..."
                        }
                    }
                },
                Some(Err(err)) => rsx! {
                    div {
                        class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded",
                        "Error loading employees: {err}"
                    }
                },
                Some(Ok(employee_list)) => rsx! {
                    div {
                        class: "bg-white shadow-md rounded-lg overflow-hidden",

                        table {
                            class: "min-w-full divide-y divide-gray-200",

                            thead {
                                class: "bg-gray-50",
                                tr {
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "ID"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "First Name"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "Last Name"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "Email"
                                    }
                                }
                            }

                            tbody {
                                class: "bg-white divide-y divide-gray-200",
                                for employee in employee_list.iter() {
                                    tr {
                                        class: "hover:bg-gray-50",
                                        td {
                                            class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                            "{employee.id}"
                                        }
                                        td {
                                            class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                            "{employee.first_name}"
                                        }
                                        td {
                                            class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                            "{employee.last_name}"
                                        }
                                        td {
                                            class: "px-6 py-4 whitespace-nowrap text-sm text-gray-600",
                                            "{employee.email}"
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
}
