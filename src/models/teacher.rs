use crate::schema::teachers;
use diesel::{Identifiable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
}
