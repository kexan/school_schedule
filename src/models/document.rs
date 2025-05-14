use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable, Selectable, prelude::Associations};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::teacher::Teacher;
use crate::schema::documents;

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations, ToSchema)]
#[diesel(belongs_to(Teacher))]
pub struct Document {
    id: Uuid,
    name: String,
    uploaded_at: NaiveDateTime,
    teacher_id: i32,
}
