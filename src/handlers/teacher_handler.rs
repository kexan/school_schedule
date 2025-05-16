use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    db::PostgresPool,
    error::AppError,
    logic::services::{document_service::DocumentService, teacher_service::TeacherService},
    models::{
        document::{Document, DocumentFileForm},
        teacher::{NewTeacher, Teacher, UpdateTeacher},
    },
};

pub fn router() -> OpenApiRouter<PostgresPool> {
    //TODO: добавить пермишены
    let dont_need_permissions = OpenApiRouter::new()
        .routes(routes!(
            create_teacher,
            get_teacher,
            update_teacher,
            delete_teacher
        ))
        .routes(routes!(upload_document));
    OpenApiRouter::new().merge(dont_need_permissions)
}

#[utoipa::path(post, path = "/", request_body = NewTeacher)]
async fn create_teacher(
    State(postgres_pool): State<PostgresPool>,
    Json(new_teacher): Json<NewTeacher>,
) -> Result<Json<Teacher>, AppError> {
    info!("Creating new student");
    let new_teacher = TeacherService::create(&postgres_pool, new_teacher)?;
    Ok(Json(new_teacher))
}

#[utoipa::path(post,
    path = "/{id}/upload_document", 
    params(("id" = i32, Path, description = "ID учителя к которому загружаем документ")),
    request_body(content_type = "multipart/form-data", content = DocumentFileForm, description = "Загружаемый документ")
)]
async fn upload_document(
    State(postgres_pool): State<PostgresPool>,
    Path(teacher_id): Path<i32>,
    multipart: Multipart,
) -> Result<Json<Document>, AppError> {
    info!("Uploading document to teacher");
    let document = DocumentService::create(&postgres_pool, multipart, teacher_id).await?;
    Ok(Json(document))
}

#[utoipa::path(get, path = "/{id}", params(("id" = i32, Path, description = "ID запрашиваемого учителя")))]
async fn get_teacher(
    State(postgres_pool): State<PostgresPool>,
    Path(teacher_id): Path<i32>,
) -> Result<Json<Teacher>, AppError> {
    info!("Getting teacher");
    let teacher = TeacherService::get(&postgres_pool, teacher_id)?;
    Ok(Json(teacher))
}

#[utoipa::path(put, path = "/{id}", params(("id" = i32, Path, description = "ID Учителя которого требуется обновить")), request_body = UpdateTeacher)]
async fn update_teacher(
    State(postgres_pool): State<PostgresPool>,
    Path(teacher_id): Path<i32>,
    Json(update_teacher): Json<UpdateTeacher>,
) -> Result<Json<Teacher>, AppError> {
    info!("Updating teacher");
    let updated_teacher = TeacherService::update(&postgres_pool, teacher_id, update_teacher)?;
    Ok(Json(updated_teacher))
}

#[utoipa::path(delete, path = "/{id}", params(("id" = i32, Path, description = "ID Учителя которого требуется удалить")))]
async fn delete_teacher(
    State(postgres_pool): State<PostgresPool>,
    Path(teacher_id): Path<i32>,
) -> Result<Json<String>, AppError> {
    info!("Deleting teacher");
    let delete_count = TeacherService::delete(&postgres_pool, teacher_id)?;
    if delete_count {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Teacher not found".to_string()))
    }
}
