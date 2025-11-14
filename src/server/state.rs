use dioxus::prelude::*;

#[derive(Clone)]
pub struct AuthState {
    pub logged_in: Signal<bool>,
}