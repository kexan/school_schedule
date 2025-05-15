use std::{fs::File, io::Write, path::Path};

use axum::extract::Multipart;

use crate::{
    db::{self, PostgresPool},
    error::AppError,
    logic::repositories::document_repository::DocumentRepository,
    models::document::{Document, NewDocument},
};

pub struct DocumentService;

impl DocumentService {
    pub async fn create(
        postgres_pool: PostgresPool,
        mut multipart: Multipart,
    ) -> Result<Document, AppError> {
        while let Some(field) = multipart.next_field().await? {
            if field.name() == Some("file") {
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
                let data = field.bytes().await?.as_ref();
                let file_extension = content_type.split('/').last().ok_or(AppError::BadRequest(
                    "Failed to determine file extension".to_string(),
                ))?;
                let file_name = field
                    .file_name()
                    .ok_or(AppError::BadRequest(
                        "Failed to determine file_name".to_string(),
                    ))?
                    .to_string();
                let new_document = NewDocument {
                    name: file_name,
                    //TODO: прокидывать тичер айди и проверять
                    teacher_id: 1,
                };
                let mut connection = db::get_postgres_connection(&postgres_pool)?;
                let database_entry = DocumentRepository::create(&mut connection, new_document)?;
                let file_name = format!("{}.{}", database_entry.id, file_extension);
                let file_path = Path::new("/storage/").join(file_name);
                let mut file = File::create(file_path)?;
                file.write_all(data)?;
                return Ok(database_entry);
            }
        }
        Err(AppError::BadRequest("No file uploaded".to_string()))
    }
}
