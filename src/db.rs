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

// CREATE Employee
#[server]
pub async fn create_employee(
    first_name: String,
    last_name: String,
    email: String,
) -> Result<Employee, ServerFnError> {
    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| ServerFnError::new("DATABASE_URL not set"))?;
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to connect to database: {}", e)))?;

    // Create employee
    let employee = sqlx::query_as::<_, Employee>(
        "INSERT INTO employee (first_name, last_name, email) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(first_name)
    .bind(last_name)
    .bind(email)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to create employee: {}", e)))?;

    Ok(employee)
}

// UPDATE Employee
#[server]
pub async fn update_employee(
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
) -> Result<Employee, ServerFnError> {
    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| ServerFnError::new("DATABASE_URL not set"))?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| ServerFnError::new(format!("Database connection failed: {}", e)))?;

    let employee = sqlx::query_as::<_, Employee>(
        "UPDATE employee SET first_name = $1, last_name = $2, email = $3 WHERE id = $4 RETURNING id, first_name, last_name, email"
    )
    .bind(&first_name)
    .bind(&last_name)
    .bind(&email)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to update employee: {}", e)))?;

    Ok(employee)
}

// DELETE Employee
#[server]
pub async fn delete_employee(id: i32) -> Result<(), ServerFnError> {
    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| ServerFnError::new("DATABASE_URL not set"))?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| ServerFnError::new(format!("Database connection failed: {}", e)))?;

    sqlx::query("DELETE FROM employee WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to delete employee: {}", e)))?;

    Ok(())
}