// src/db.rs
// Unified DB API: Employees + UsersExtended (view)
// Code & comments in English.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

//
// Server-only imports
//
#[cfg(feature = "server")]
use sea_orm::*;

#[cfg(feature = "server")]
use crate::db_connection::get_db;

#[cfg(feature = "server")]
pub use crate::entities::prelude::Employee as EmployeeEntity;

#[cfg(feature = "server")]
pub use crate::entities::employee;

//
// -------------------- EMPLOYEE DTO --------------------
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Employee {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[cfg(feature = "server")]
impl From<employee::Model> for Employee {
    fn from(m: employee::Model) -> Self {
        Self {
            id: m.id,
            first_name: m.first_name,
            last_name: m.last_name,
            email: m.email,
        }
    }
}

//
// -------------------- EMPLOYEE CRUD --------------------
//
#[server]
pub async fn get_employees_server() -> Result<Vec<Employee>, ServerFnError> {
    let db = get_db()
        .await
        .map_err(|e| ServerFnError::new(format!("DB connection failed: {}", e)))?;

    let models = EmployeeEntity::find()
        .all(&db)
        .await
        .map_err(|e| ServerFnError::new(format!("Query failed: {}", e)))?;

    Ok(models.into_iter().map(Into::into).collect())
}

#[server]
pub async fn create_employee(
    first_name: String,
    last_name: String,
    email: String,
) -> Result<Employee, ServerFnError> {
    let db = get_db()
        .await
        .map_err(|e| ServerFnError::new(format!("DB connection failed: {}", e)))?;

    let new = employee::ActiveModel {
        id: NotSet,
        first_name: Set(Some(first_name)),
        last_name: Set(Some(last_name)),
        email: Set(Some(email)),
    };

    let res = new
        .insert(&db)
        .await
        .map_err(|e| ServerFnError::new(format!("Insert failed: {}", e)))?;

    Ok(res.into())
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
        .map_err(|e| ServerFnError::new(format!("DB connection failed: {}", e)))?;

    let existing = EmployeeEntity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| ServerFnError::new(format!("Query failed: {}", e)))?
        .ok_or_else(|| ServerFnError::new("Employee not found"))?;

    let mut model: employee::ActiveModel = existing.into();
    model.first_name = Set(Some(first_name));
    model.last_name = Set(Some(last_name));
    model.email = Set(Some(email));

    let updated = model
        .update(&db)
        .await
        .map_err(|e| ServerFnError::new(format!("Update failed: {}", e)))?;

    Ok(updated.into())
}

#[server]
pub async fn delete_employee(id: i32) -> Result<(), ServerFnError> {
    let db = get_db()
        .await
        .map_err(|e| ServerFnError::new(format!("DB connection failed: {}", e)))?;

    EmployeeEntity::delete_by_id(id)
        .exec(&db)
        .await
        .map_err(|e| ServerFnError::new(format!("Delete failed: {}", e)))?;

    Ok(())
}

//
// -------------------- USERS_EXTENDED VIEW (FROM DATABASE VIEW) --------------------
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserExtended {
    pub user_id: i32,
    pub email: String,
    pub role_name: String,
    pub employee_id: Option<i32>,
    pub employee_name: Option<String>,
}

#[server]
pub async fn get_users_extended_server() -> Result<Vec<UserExtended>, ServerFnError> {
    // We use sea-orm's FromQueryResult + Statement::from_sql_and_values to read the view.
    // This avoids using QueryResult/try_get and is fully compatible with Dioxus server macro.

    #[cfg(feature = "server")]
    {
        use sea_orm::FromQueryResult;

        #[derive(Debug, FromQueryResult)]
        struct Row {
            user_id: i32,
            email: String,
            role_name: String,
            employee_id: Option<i32>,
            employee_name: Option<String>,
        }

        let db = get_db()
            .await
            .map_err(|e| ServerFnError::new(format!("DB connection failed: {}", e)))?;

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT
                user_id,
                email,
                role_name,
                employee_id,
                employee_name
            FROM users_extended
            ORDER BY user_id
            "#,
            [], // no params
        );

        let rows: Vec<Row> = Row::find_by_statement(stmt)
            .all(&db)
            .await
            .map_err(|e| ServerFnError::new(format!("Query failed: {}", e)))?;

        let mapped = rows
            .into_iter()
            .map(|r| UserExtended {
                user_id: r.user_id,
                email: r.email,
                role_name: r.role_name,
                employee_id: r.employee_id,
                employee_name: r.employee_name,
            })
            .collect();

        Ok(mapped)
    }

    // If compiled without server feature (shouldn't happen in server context),
    // return an empty vector.
    #[cfg(not(feature = "server"))]
    {
        Ok(Vec::new())
    }
}
