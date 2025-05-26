use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    AppState,
    error::AppError,
    logic::services::{document_service::DocumentService, teacher_service::TeacherService},
    models::{
        document::{Document, DocumentFileForm},
        teacher::{NewTeacher, Teacher, UpdateTeacher},
    },
};

pub fn router() -> OpenApiRouter<AppState> {
    //TODO: добавить пермишены
    let dont_need_permissions = OpenApiRouter::new()
        .routes(routes!(
            create_teacher,
            get_teacher,
            update_teacher,
            delete_teacher
        ))
        .routes(routes!(
            get_teacher_documents,
            upload_document,
            delete_document
        ));
    OpenApiRouter::new().merge(dont_need_permissions)
}

#[utoipa::path(post, path = "/", request_body = NewTeacher)]
async fn create_teacher(
    State(teacher_service): State<TeacherService>,
    Json(new_teacher): Json<NewTeacher>,
) -> Result<Json<Teacher>, AppError> {
    info!("Creating new teacher");
    let new_teacher = teacher_service.create(new_teacher)?;
    Ok(Json(new_teacher))
}

#[utoipa::path(
    post,
    path = "/{id}/documents",
    params(("id" = i32, Path, description = "ID учителя к которому загружаем документ")),
    request_body(content_type = "multipart/form-data", content = DocumentFileForm, description = "Загружаемый документ")
)]
async fn upload_document(
    State(document_service): State<DocumentService>,
    Path(teacher_id): Path<i32>,
    multipart: Multipart,
) -> Result<Json<Document>, AppError> {
    info!("Uploading document to teacher with ID {}", teacher_id);
    let document = document_service.create(multipart, teacher_id).await?;
    Ok(Json(document))
}

#[utoipa::path(get, path = "/{id}", params(("id" = i32, Path, description = "ID запрашиваемого учителя")))]
async fn get_teacher(
    State(teacher_service): State<TeacherService>,
    Path(teacher_id): Path<i32>,
) -> Result<Json<Teacher>, AppError> {
    info!("Getting teacher with ID {}", teacher_id);
    let teacher = teacher_service.get(teacher_id)?;
    Ok(Json(teacher))
}

#[utoipa::path(get, path = "/{id}/documents", params(("id" = i32, Path, description = "ID учителя у которого запрашиваются документы")))]
async fn get_teacher_documents(
    State(document_service): State<DocumentService>,
    Path(teacher_id): Path<i32>,
) -> Result<Json<Vec<Document>>, AppError> {
    info!("Getting documents for teacher with ID {}", teacher_id);
    let documents = document_service.get_by_teacher_id(teacher_id)?;
    Ok(Json(documents))
}

#[utoipa::path(put, path = "/{id}", params(("id" = i32, Path, description = "ID Учителя которого требуется обновить")), request_body = UpdateTeacher)]
async fn update_teacher(
    State(teacher_service): State<TeacherService>,
    Path(teacher_id): Path<i32>,
    Json(update_teacher): Json<UpdateTeacher>,
) -> Result<Json<Teacher>, AppError> {
    info!("Updating teacher with ID {}", teacher_id);
    let updated_teacher = teacher_service.update(teacher_id, update_teacher)?;
    Ok(Json(updated_teacher))
}

#[utoipa::path(delete, path = "/{id}", params(("id" = i32, Path, description = "ID Учителя которого требуется удалить")))]
async fn delete_teacher(
    State(teacher_service): State<TeacherService>,
    Path(teacher_id): Path<i32>,
) -> Result<Json<String>, AppError> {
    info!("Deleting teacher with ID {}", teacher_id);
    let deleted = teacher_service.delete(teacher_id)?;
    if deleted {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Teacher not found".to_string()))
    }
}

#[utoipa::path(
    delete,
    path = "/{id}/documents/{document_id}",
    params(
        ("id" = i32, Path, description = "ID учителя"),
        ("document_id" = Uuid, Path, description = "ID документа который нужно удалить")
    ),
    responses(
        (status = 200, description = "Документ удален", body = String)
    )
)]
async fn delete_document(
    State(document_service): State<DocumentService>,
    Path((teacher_id, document_id)): Path<(i32, Uuid)>,
) -> Result<Json<String>, AppError> {
    info!(
        "Deleting document {} from teacher with ID {}",
        document_id, teacher_id
    );
    let deleted = document_service.delete(document_id)?;
    if deleted {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Document not found".to_string()))
    }
}
