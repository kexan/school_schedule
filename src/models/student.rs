use chrono::NaiveDate;
use diesel::{
    Selectable,
    prelude::{AsChangeset, Associations, Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{parent::Parent, student_group::StudentGroup},
    schema::students,
};

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Parent))]
#[diesel(belongs_to(StudentGroup))]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub birth_date: NaiveDate,
    pub parent_id: Option<i32>,
    pub student_group_id: Option<i32>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = students)]
pub struct NewStudent<'a> {
    pub name: &'a str,
    pub birth_date: NaiveDate,
    pub parent_id: Option<i32>,
    pub student_group_id: Option<i32>,
}
