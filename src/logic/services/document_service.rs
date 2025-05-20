use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

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
            let allowed_types = ["image/png", "image/jpeg", "image/webp", "application/pdf"];
            if !allowed_types.contains(&content_type.as_str()) {
                return Err(AppError::BadRequest(
                    "Unsupported file type. Only png/jpeg/webp/pdf allowed.".to_string(),
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

            let file_name = format!("{}.{}", database_entry.id, database_entry.file_extension()?);
            let dir_path = format!("./storage/teachers/{}/", database_entry.teacher_id);
            let storage_dir = Path::new(&dir_path);

            if let Err(e) = std::fs::create_dir_all(storage_dir) {
                DocumentService::delete(postgres_pool, database_entry.id)?;
                return Err(AppError::InternalServerError(e.to_string()));
            }

            let file_path = storage_dir.join(file_name);
            let mut file = match File::create(&file_path) {
                Ok(f) => f,
                Err(e) => {
                    DocumentService::delete(postgres_pool, database_entry.id)?;
                    return Err(AppError::InternalServerError(e.to_string()));
                }
            };

            let data = field.bytes().await?;
            let data_vec = data.to_vec();
            if let Err(e) = file.write_all(&data_vec) {
                DocumentService::delete(postgres_pool, database_entry.id)?;
                return Err(AppError::InternalServerError(e.to_string()));
            }

            info!("Successfully saved document");
            Ok(database_entry)
        } else {
            Err(AppError::BadRequest("No file uploaded".to_string()))
        }
    }

    pub fn get(postgres_pool: &PostgresPool, document_id: Uuid) -> Result<Document, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let document = DocumentRepository::get(&mut connection, document_id)?;
        info!("Document with ID {} successfully get", document_id);
        Ok(document)
    }

    pub fn get_all_for_teacher(
        postgres_pool: &PostgresPool,
        teacher_id: i32,
    ) -> Result<Vec<Document>, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let documents = DocumentRepository::get_by_teacher_id(&mut connection, teacher_id)?;
        info!("Got all documents for teacher with ID {}", teacher_id);
        Ok(documents)
    }

    pub fn delete(postgres_pool: &PostgresPool, document_id: Uuid) -> Result<bool, AppError> {
        let document = DocumentService::get(postgres_pool, document_id)?;

        let file_extension = document.file_extension()?;
        let teacher_id = document.teacher_id;

        let dir_path = format!("./storage/teachers/{}/", teacher_id);
        let file_name = document_id.to_string() + "." + file_extension;
        let full_path = Path::new(&dir_path).join(file_name);

        if full_path.exists() {
            fs::remove_file(&full_path)?;
            info!("File at {} successfully deleted", full_path.display());

            let mut connection = db::get_postgres_connection(postgres_pool)?;
            let deleted_count = DocumentRepository::delete(&mut connection, document_id)?;

            if deleted_count > 0 {
                info!("Document with ID {} successfully deleted", document_id);
                Ok(true)
            } else {
                info!("Document with ID {} not found", document_id);
                Ok(false)
            }
        } else {
            info!("File at {} not found", full_path.display());
            Ok(false)
        }
    }
}
