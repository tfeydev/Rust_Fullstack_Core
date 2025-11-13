use dioxus::prelude::*;
use crate::db::{UserExtended, get_users_extended_server};

#[component]
pub fn Users() -> Element {
    let mut users = use_resource(move || async move { get_users_extended_server().await });

    rsx! {
        div {
            class: "container mx-auto px-4 py-8",

            // Header
            div {
                class: "flex justify-between items-center mb-6",
                h1 {
                    class: "text-3xl font-bold text-gray-800",
                    "Users"
                }
            }

            match users() {
                None => rsx! {
                    div {
                        class: "flex justify-center items-center py-12",
                        p { class: "text-gray-600", "Loading users..." }
                    }
                },
                Some(Err(err)) => rsx! {
                    div {
                        class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded",
                        "Error loading users: {err}"
                    }
                },
                Some(Ok(user_list)) => rsx! {
                    div {
                        class: "bg-white shadow-md rounded-lg overflow-hidden",

                        table {
                            class: "min-w-full divide-y divide-gray-200",

                            thead {
                                class: "bg-gray-50",
                                tr {
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "ID" }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "Email" }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "Role" }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "Employee" }
                                }
                            }

                            tbody {
                                class: "bg-white divide-y divide-gray-200",

                                for user in user_list.iter() {
                                    tr {
                                        key: "{user.user_id}",
                                        class: "hover:bg-gray-50",

                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{user.user_id}" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{user.email}" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{user.role_name}" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-600",
                                            "{user.employee_name.clone().unwrap_or(\"-\".into())}"
                                        }
                                    }
                                }
                            }
                        }

                        if user_list.is_empty() {
                            div {
                                class: "text-center py-8 text-gray-500",
                                "No users found."
                            }
                        }
                    }
                }
            }
        }
    }
}
