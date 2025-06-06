use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, DbEnum, Serialize, Deserialize, ToSchema)]
#[db_enum(existing_type_path = "crate::schema::sql_types::PermissionRole")]
pub enum PermissionRole {
    User,
    Teacher,
    Director,
    Admin,
}
