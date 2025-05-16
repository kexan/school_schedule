use std::{fs::File, io::Write, path::Path};

use axum::extract::Multipart;
use tracing::info;
use uuid::Uuid;

use crate::{
    db::{self, PostgresPool},
    error::AppError,
    logic::repositories::document_repository::DocumentRepository,
    models::document::{Document, NewDocument},
};

pub struct DocumentService;

impl DocumentService {
    pub async fn create(
        postgres_pool: &PostgresPool,
        mut multipart: Multipart,
        teacher_id: i32,
    ) -> Result<Document, AppError> {
        if let Some(field) = multipart.next_field().await? {
            let content_type = field
                .content_type()
                .ok_or(AppError::BadRequest("Content type is missing".to_string()))?
                .to_string();
            let allowed_types = ["image/png", "image/jpeg", "image/webp"];
            if !allowed_types.contains(&content_type.as_str()) {
                return Err(AppError::BadRequest(
                    "Unsupported file type. Only png/jpeg/webp allowed.".to_string(),
                ));
            }

            let name = field
                .file_name()
                .ok_or(AppError::BadRequest(
                    "Failed to determine file_name".to_string(),
                ))?
                .to_string();

            let new_document = NewDocument { name, teacher_id };
            let mut connection = db::get_postgres_connection(postgres_pool)?;
            let database_entry = DocumentRepository::create(&mut connection, new_document)?;

            let file_extension =
                content_type
                    .split('/')
                    .next_back()
                    .ok_or(AppError::BadRequest(
                        "Failed to determine file extension".to_string(),
                    ))?;
            let file_name = format!("{}.{}", database_entry.id, file_extension);
            let file_path = Path::new("/storage/").join(file_name);
            let mut file = File::create(file_path)?;
            let data = field.bytes().await?.to_vec();
            file.write_all(&data)?;

            info!("Successfully saved document");
            return Ok(database_entry);
        }
        Err(AppError::BadRequest("No file uploaded".to_string()))
    }

    pub fn get(postgres_pool: &PostgresPool, document_id: Uuid) -> Result<Document, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let document = DocumentRepository::get(&mut connection, document_id)?;
        info!("Document with ID {} successfully get", document_id);
        Ok(document)
    }

    pub fn delete(postgres_pool: &PostgresPool, document_id: Uuid) -> Result<bool, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let deleted_count = DocumentRepository::delete(&mut connection, document_id)?;

        if deleted_count > 0 {
            info!("Document with ID {} successfully deleted", document_id);
            Ok(true)
        } else {
            info!("Document with ID {} not found", document_id);
            Ok(false)
        }
    }
}
