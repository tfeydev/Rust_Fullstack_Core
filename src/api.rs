use axum::{Router, routing::get, extract::State, Json};
use sqlx::PgPool;
use std::sync::Arc;
use crate::db;

pub fn router(db: Arc<PgPool>) -> Router {
    Router::new()
        .route("/employees", get(list_employees))
        .with_state(db)
}

async fn list_employees(State(db): State<Arc<PgPool>>) -> Json<Vec<db::Employee>> {
    match db::get_employees(&db).await {
        Ok(list) => Json(list),
        Err(err) => {
            eprintln!("DB error: {err}");
            Json(vec![])
        }
    }
}
