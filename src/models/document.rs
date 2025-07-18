use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable};
use diesel::{Identifiable, Queryable, Selectable, prelude::Associations};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
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

impl Document {
    pub fn file_extension(&self) -> Result<&str, AppError> {
        self.name
            .split('.')
            .next_back()
            .ok_or(AppError::InternalServerError(
                "Could not get document file extension".to_string(),
            ))
    }
}

#[derive(Insertable, AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = documents)]
pub struct NewDocument {
    pub name: String,
    pub teacher_id: i32,
}

// Структура для сваггера
#[derive(Deserialize, ToSchema)]
pub struct DocumentFileForm {
    #[schema(format = Binary, content_media_type = "application/octet-stream")]
    pub document: String,
}
