use diesel::{
    Selectable,
    prelude::{AsChangeset, Associations, Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    models::{lesson::Lesson, student::Student},
    schema::attendances,
};

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations, ToSchema)]
#[diesel(belongs_to(Lesson))]
#[diesel(belongs_to(Student))]
pub struct Attendance {
    pub id: i32,
    pub student_id: i32,
    pub lesson_id: i32,
    pub is_present: bool,
    pub skip_reason: Option<String>,
}

#[derive(Insertable, AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = attendances)]
pub struct NewAttendance {
    pub student_id: i32,
    pub lesson_id: i32,
    pub is_present: bool,
    pub skip_reason: Option<String>,
}

#[derive(Insertable, AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = attendances)]
pub struct UpdateAttendance {
    pub is_present: Option<bool>,
    pub skip_reason: Option<String>,
}
