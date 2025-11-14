mod app;
mod components;
mod server;
mod entities;

fn main() {
    dotenvy::dotenv().ok();
    dioxus::launch(app::App);
}
