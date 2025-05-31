use axum::{
    Json,
    extract::{Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    error::AppError,
    logic::services::student_service::StudentService,
    models::student::{NewStudent, StudentWithRelations, UpdateStudent},
};

pub fn router() -> OpenApiRouter<AppState> {
    let dont_need_permissions = OpenApiRouter::new().routes(routes!(
        create_student,
        get_student,
        update_student,
        delete_student
    ));
    OpenApiRouter::new().merge(dont_need_permissions)
}

#[utoipa::path(post, path = "/", request_body = NewStudent)]
async fn create_student(
    State(student_service): State<StudentService>,
    Json(new_student): Json<NewStudent>,
) -> Result<Json<StudentWithRelations>, AppError> {
    info!("Creating new student");
    let new_student = student_service.create(new_student)?;
    Ok(Json(new_student))
}

#[utoipa::path(get, path = "/{id}", params(("id" = i32, Path, description = "ID запрашиваемого ученика")))]
async fn get_student(
    State(student_service): State<StudentService>,
    Path(student_id): Path<i32>,
) -> Result<Json<StudentWithRelations>, AppError> {
    info!("Getting student");
    let student = student_service.get(student_id)?;
    Ok(Json(student))
}

#[utoipa::path(put, path = "/{id}", params(("id" = i32, Path, description = "ID Ученика которого требуется обновить")), request_body = UpdateStudent)]
async fn update_student(
    State(student_service): State<StudentService>,
    Path(student_id): Path<i32>,
    Json(update_student): Json<UpdateStudent>,
) -> Result<Json<StudentWithRelations>, AppError> {
    info!("Updating student");
    let updated_student = student_service.update(student_id, update_student)?;
    Ok(Json(updated_student))
}

#[utoipa::path(delete, path = "/{id}", params(("id" = i32, Path, description = "ID Ученика которого требуется удалить")))]
async fn delete_student(
    State(student_service): State<StudentService>,
    Path(student_id): Path<i32>,
) -> Result<Json<String>, AppError> {
    info!("Deleting student");
    let deleted = student_service.delete(student_id)?;
    if deleted {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Student not found".to_string()))
    }
}
