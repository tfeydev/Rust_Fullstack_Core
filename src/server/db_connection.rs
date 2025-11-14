//! Database connection setup for SeaORM
#[cfg(feature = "server")]
use sea_orm::{Database, DatabaseConnection, DbErr};

/// Get database connection from environment variable
#[allow(dead_code)]
#[cfg(feature = "server")]
pub async fn get_db() -> Result<DatabaseConnection, DbErr> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    Database::connect(&database_url).await
}

/// Get database connection or panic (for use in main/startup)
#[cfg(feature = "server")]
#[allow(dead_code)]
pub async fn establish_connection() -> DatabaseConnection {
    get_db().await.expect("Failed to connect to database")
}
