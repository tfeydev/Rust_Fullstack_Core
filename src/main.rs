// use dioxus::prelude::*;

mod app;
mod components;

mod db;

fn main() {
    dotenvy::dotenv().ok();
    dioxus::launch(app::App);
}
