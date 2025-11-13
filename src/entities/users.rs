// SeaORM entity for "users"
use sea_orm::entity::prelude::*;

// Model definition for the users table
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    // Primary key with auto increment (sequence)
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    // Unique email used for login
    #[sea_orm(unique)]
    pub email: String,

    // bcrypt password hash
    pub password_hash: String,

    // FK to app_role.id
    pub role_id: i32,

    // Optional creation timestamp
    pub created_at: Option<DateTimeWithTimeZone>,

    // Optional FK to employee.id (unique enforces one-to-one mapping)
    #[sea_orm(unique)]
    pub employee_id: Option<i32>,
}

// Relations from users to other entities
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::app_role::Entity",
        from = "Column::RoleId",
        to = "super::app_role::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AppRole,

    #[sea_orm(
        belongs_to = "super::employee::Entity",
        from = "Column::EmployeeId",
        to = "super::employee::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Employee,
}

impl Related<super::app_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AppRole.def()
    }
}

impl Related<super::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

// Default ActiveModel behavior
impl ActiveModelBehavior for ActiveModel {}
