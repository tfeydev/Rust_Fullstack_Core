mod app;
mod components;

mod db;

mod entities;

#[cfg(feature = "server")]
mod db_connection;

fn main() {
    dotenvy::dotenv().ok();
    dioxus::launch(app::App);
}
