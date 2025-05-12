use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::parents;

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, ToSchema)]
pub struct Parent {
    id: i32,
    name: String,
    additional_info: Option<String>,
}

#[derive(Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = parents)]
pub struct NewParent {
    name: String,
    additional_info: Option<String>,
}

#[derive(Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = parents)]
pub struct UpdateParent {
    name: Option<String>,
    additional_info: Option<String>,
}
