use dioxus::prelude::*;
use crate::db::get_users_extended_server;

#[component]
pub fn Users() -> Element {
    let users = use_resource(|| async move {
        get_users_extended_server().await.ok()
    });

    rsx! {
        div {
            class: "container mx-auto px-4 py-8",

            // Header
            h1 {
                class: "text-3xl font-bold text-blue-500 mb-6",
                "Users"
            }

            // Table Wrapper
            div {
                class: "bg-white shadow-md rounded-lg overflow-hidden",

                match &*users.read() {
                    None => rsx! {
                        div { class: "flex justify-center items-center py-12",
                            p { class: "text-gray-600", "Loading users..." }
                        }
                    },

                    Some(None) => rsx! {
                        div { class: "p-6 text-red-600", "Error loading users" }
                    },

                    Some(Some(list)) => rsx! {
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

                                for u in list.iter() {
                                    tr {
                                        key: "{u.user_id}",
                                        class: "hover:bg-gray-50",

                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{u.user_id}" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{u.email}" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900", "{u.role_name}" }

                                        td {
                                            class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                            "{u.employee_name.clone().unwrap_or(\"-\".into())}"
                                        }
                                    }
                                }
                            }
                        }

                        if list.is_empty() {
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
