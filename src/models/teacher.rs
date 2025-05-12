use crate::schema::teachers;
use diesel::{
    Identifiable, Queryable, Selectable,
    prelude::{AsChangeset, Insertable},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, ToSchema)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = teachers)]
pub struct NewTeacher {
    pub name: String,
}

#[derive(Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = teachers)]
pub struct UpdateTeacher {
    pub name: Option<String>,
}
