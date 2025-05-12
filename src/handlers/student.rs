use axum::{
    Json,
    extract::{Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    db::PostgresPool,
    error::AppError,
    logic::services::student::StudentService,
    models::student::{NewStudent, Student, UpdateStudent},
};

pub fn router() -> OpenApiRouter<PostgresPool> {
    //TODO: добавить пермишены
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
    State(postgres_pool): State<PostgresPool>,
    Json(new_student): Json<NewStudent>,
) -> Result<Json<Student>, AppError> {
    info!("Creating new student");
    let new_student = StudentService::create(&postgres_pool, new_student)?;
    Ok(Json(new_student))
}

#[utoipa::path(get, path = "/{id}", params(("id" = i32, Path, description = "ID запрашиваемого ученика")))]
async fn get_student(
    State(postgres_pool): State<PostgresPool>,
    Path(student_id): Path<i32>,
) -> Result<Json<Student>, AppError> {
    info!("Getting student");
    let student = StudentService::get(&postgres_pool, student_id)?;
    Ok(Json(student))
}

#[utoipa::path(put, path = "/{id}", params(("id" = i32, Path, description = "ID Ученика которого требуется обновить")), request_body = UpdateStudent)]
async fn update_student(
    State(postgres_pool): State<PostgresPool>,
    Path(student_id): Path<i32>,
    Json(update_student): Json<UpdateStudent>,
) -> Result<Json<Student>, AppError> {
    info!("Updating student");
    let updated_student = StudentService::update(&postgres_pool, student_id, update_student)?;
    Ok(Json(updated_student))
}

#[utoipa::path(delete, path = "/{id}", params(("id" = i32, Path, description = "ID Ученика которого требуется удалить")))]
async fn delete_student(
    State(postgres_pool): State<PostgresPool>,
    Path(student_id): Path<i32>,
) -> Result<Json<String>, AppError> {
    info!("Deleting student");
    let delete_count = StudentService::delete(&postgres_pool, student_id)?;
    if delete_count {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Student not found".to_string()))
    }
}
