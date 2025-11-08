use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// Re-export entities for easier access (server-only)
#[cfg(feature = "server")]
pub use crate::entities::employee;

#[cfg(feature = "server")]
pub use crate::entities::prelude::{Employee as EmployeeEntity, Users as UsersEntity, AppRole as AppRoleEntity};

// Employee struct - but we need a client-safe version!
// Since employee::Model only exists on server, we need to define it for both
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Employee {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

// Server-only: Convert from SeaORM Model to our Employee struct
#[cfg(feature = "server")]
impl From<employee::Model> for Employee {
    fn from(model: employee::Model) -> Self {
        Employee {
            id: model.id,
            first_name: model.first_name,
            last_name: model.last_name,
            email: model.email,
        }
    }
}

// Server-only database functions
#[cfg(feature = "server")]
use sea_orm::*;

#[cfg(feature = "server")]
use crate::db_connection::get_db;

// Server Function - Get all employees
#[server]
pub async fn get_employees_server() -> Result<Vec<Employee>, ServerFnError> {
    let db = get_db()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to connect to database: {}", e)))?;

    let employees = EmployeeEntity::find()
        .all(&db)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch employees: {}", e)))?;

    // Convert SeaORM models to our Employee struct
    let employees: Vec<Employee> = employees.into_iter().map(Into::into).collect();

    Ok(employees)
}

// CREATE Employee
#[server]
pub async fn create_employee(
    first_name: String,
    last_name: String,
    email: String,
) -> Result<Employee, ServerFnError> {
    let db = get_db()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to connect to database: {}", e)))?;

    let employee = employee::ActiveModel {
        first_name: Set(Some(first_name)),
        last_name: Set(Some(last_name)),
        email: Set(Some(email)),
        ..Default::default()
    };

    let result = employee
        .insert(&db)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to create employee: {}", e)))?;

    Ok(result.into())
}

// UPDATE Employee
#[server]
pub async fn update_employee(
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
) -> Result<Employee, ServerFnError> {
    let db = get_db()
        .await
        .map_err(|e| ServerFnError::new(format!("Database connection failed: {}", e)))?;

    // Find existing employee
    let employee = EmployeeEntity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to find employee: {}", e)))?
        .ok_or_else(|| ServerFnError::new("Employee not found"))?;

    // Convert to ActiveModel for updating
    let mut employee: employee::ActiveModel = employee.into();
    
    employee.first_name = Set(Some(first_name));
    employee.last_name = Set(Some(last_name));
    employee.email = Set(Some(email));

    let updated = employee
        .update(&db)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to update employee: {}", e)))?;

    Ok(updated.into())
}

// DELETE Employee
#[server]
pub async fn delete_employee(id: i32) -> Result<(), ServerFnError> {
    let db = get_db()
        .await
        .map_err(|e| ServerFnError::new(format!("Database connection failed: {}", e)))?;

    EmployeeEntity::delete_by_id(id)
        .exec(&db)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to delete employee: {}", e)))?;

    Ok(())
}
