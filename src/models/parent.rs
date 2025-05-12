use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};

use crate::schema::parents;

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable)]
pub struct Parent {
    id: i32,
    name: String,
    additional_info: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = parents)]
pub struct NewParent<'a> {
    name: &'a str,
    additional_info: Option<&'a str>,
}
