use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::parents;

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, ToSchema)]
pub struct Parent {
    pub id: i32,
    pub name: String,
    pub additional_info: Option<String>,
}

#[derive(Insertable, AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = parents)]
pub struct NewParent {
    pub name: String,
    pub additional_info: Option<String>,
}

#[derive(Insertable, AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = parents)]
pub struct UpdateParent {
    pub name: Option<String>,
    pub additional_info: Option<String>,
}
