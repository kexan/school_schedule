use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable};
use diesel::{Identifiable, Queryable, Selectable, prelude::Associations};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::teacher::Teacher;
use crate::schema::documents;

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations, ToSchema)]
#[diesel(belongs_to(Teacher))]
pub struct Document {
    pub id: Uuid,
    pub name: String,
    pub uploaded_at: NaiveDateTime,
    pub teacher_id: i32,
}

#[derive(Insertable, AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = documents)]
pub struct NewDocument {
    pub name: String,
    pub teacher_id: i32,
}
