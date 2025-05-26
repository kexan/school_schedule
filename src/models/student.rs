use chrono::NaiveDate;
use diesel::{
    Selectable,
    prelude::{AsChangeset, Associations, Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    models::{parent::Parent, student_group::StudentGroup},
    schema::students,
};

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations, ToSchema)]
#[diesel(belongs_to(Parent))]
#[diesel(belongs_to(StudentGroup))]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub birth_date: NaiveDate,
    #[serde(skip_serializing)]
    pub parent_id: Option<i32>,
    #[serde(skip_serializing)]
    pub student_group_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct StudentWithRelations {
    #[serde(flatten)]
    pub student: Student,
    pub parent: Option<Parent>,
    pub student_group: Option<StudentGroup>,
}

#[derive(Insertable, AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = students)]
pub struct NewStudent {
    pub name: String,
    pub birth_date: NaiveDate,
    pub parent_id: Option<i32>,
    pub student_group_id: Option<i32>,
}

#[derive(Insertable, AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = students)]
pub struct UpdateStudent {
    pub name: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub parent_id: Option<i32>,
    pub student_group_id: Option<i32>,
}
