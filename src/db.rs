use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// Employee struct is needed on both client and server
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// Server-only database functions
#[cfg(feature = "server")]
use sqlx::{PgPool, postgres::PgPoolOptions};

#[cfg(feature = "server")]
pub async fn get_employees(pool: &PgPool) -> anyhow::Result<Vec<Employee>> {
    let query = "SELECT id, first_name, last_name, email FROM employee";
    let employees = sqlx::query_as::<_, Employee>(query).fetch_all(pool).await?;
    Ok(employees)
}

// Server Function - available on both client (as RPC call) and server (as implementation)
#[server]
pub async fn get_employees_server() -> Result<Vec<Employee>, ServerFnError> {
    // Connect to database
    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| ServerFnError::new("DATABASE_URL not set"))?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to connect to database: {}", e)))?;

    // Get employees
    let employees = get_employees(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch employees: {}", e)))?;

    Ok(employees)
}
