use diesel::{
    Selectable,
    prelude::{AsChangeset, Associations, Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::teacher::Teacher;
use crate::schema::student_groups;

#[derive(Serialize, Deserialize, Selectable, Identifiable, Associations, Queryable)]
#[diesel(belongs_to(Teacher))]
pub struct StudentGroup {
    pub id: i32,
    pub direction: Option<String>,
    pub free_spots: i32,
    pub teacher_id: Option<i32>,
}

#[derive(Insertable, AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = student_groups)]
pub struct NewStudentGroup {
    pub direction: Option<String>,
    pub free_spots: i32,
    pub teacher_id: Option<i32>,
}

#[derive(Insertable, AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = student_groups)]
pub struct UpdateStudentGroup {
    pub direction: Option<String>,
    pub free_spots: Option<i32>,
    pub teacher_id: Option<i32>,
}
