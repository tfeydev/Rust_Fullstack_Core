use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("ERROR: DATABASE_URL not set in .env file or environment. Please set it to a valid PostgreSQL connection string.");
    
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("ERROR: Failed to create PostgreSQL connection pool. Check DATABASE_URL and if PostgreSQL is running.")
}