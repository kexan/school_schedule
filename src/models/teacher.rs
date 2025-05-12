use crate::schema::teachers;
use diesel::{
    Identifiable, Queryable, Selectable,
    prelude::{AsChangeset, Insertable},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = teachers)]
pub struct NewTeacher<'a> {
    pub name: &'a str,
}
