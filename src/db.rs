use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
pub use crate::entities::prelude::{Employee as EmployeeEntity, Users as UsersEntity, AppRole as AppRoleEntity};
#[cfg(feature = "server")]
pub use crate::entities::employee;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Employee {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[cfg(feature = "server")]
use sea_orm::*;
#[cfg(feature = "server")]
use crate::db_connection::get_db;

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

#[server]
pub async fn get_employees_server() -> Result<Vec<Employee>, ServerFnError> {
    let db = get_db()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to connect to database: {}", e)))?;

    let employees = EmployeeEntity::find()
        .all(&db)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch employees: {}", e)))?;

    Ok(employees.into_iter().map(Into::into).collect())
}

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
        id: NotSet,
        first_name: Set(Some(first_name)),
        last_name: Set(Some(last_name)),
        email: Set(Some(email)),
    };

    let result = employee
        .insert(&db)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to create employee: {}", e)))?;

    Ok(result.into())
}

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

    let employee = EmployeeEntity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to find employee: {}", e)))?
        .ok_or_else(|| ServerFnError::new("Employee not found"))?;

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
