use chrono::NaiveDate;
use diesel::{
    AsChangeset, Identifiable, Queryable, Selectable,
    prelude::{Associations, Insertable},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{models::student_group::StudentGroup, schema::lessons};

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations, ToSchema)]
#[diesel(belongs_to(StudentGroup))]
pub struct Lesson {
    pub id: i32,
    pub topic: String,
    pub scheduled_at: NaiveDate,
    pub student_group_id: Option<i32>,
}

#[derive(Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = lessons)]
pub struct NewLesson {
    pub topic: String,
    pub scheduled_at: NaiveDate,
    pub student_group_id: Option<i32>,
}

#[derive(Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = lessons)]
pub struct UpdateLesson {
    pub topic: Option<String>,
    pub scheduled_at: Option<NaiveDate>,
    pub student_group_id: Option<i32>,
}
