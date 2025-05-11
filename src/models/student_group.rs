use diesel::{
    Selectable,
    prelude::{Associations, Identifiable, Queryable},
};
use serde::{Deserialize, Serialize};

use crate::models::teacher::Teacher;
use crate::schema::student_groups;

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Teacher))]
pub struct StudentGroup {
    pub id: i32,
    pub direction: String,
    pub free_spots: i32,
    pub teacher_id: i32,
}
