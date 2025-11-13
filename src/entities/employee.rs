// SeaORM entity for "employee"
use sea_orm::entity::prelude::*;

// Model definition for the employee table
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "employee")]
pub struct Model {
    // Primary key with auto increment (sequence)
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    // Optional first name
    pub first_name: Option<String>,

    // Optional last name
    pub last_name: Option<String>,

    // Optional unique email for employee
    #[sea_orm(unique)]
    pub email: Option<String>,
}

// Relations from employee to other entities
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // one employee may have one user (user.employee_id references employee.id)
    #[sea_orm(has_one = "super::users::Entity")]
    User,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

// Default ActiveModel behavior
impl ActiveModelBehavior for ActiveModel {}
