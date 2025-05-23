use axum::{
    Json,
    extract::{Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    db::PostgresPool,
    error::AppError,
    logic::services::{lesson_service::LessonService, student_group_service::StudentGroupService},
    models::{
        lesson::{Lesson, NewLesson},
        student_group::{NewStudentGroup, StudentGroup, UpdateStudentGroup},
    },
};

pub fn router() -> OpenApiRouter<PostgresPool> {
    //TODO: добавить пермишены
    let dont_need_permissions = OpenApiRouter::new()
        .routes(routes!(
            create_student_group,
            get_student_group,
            update_student_group,
            delete_student_group
        ))
        .routes(routes!(
            get_lessons_for_student_group,
            create_lesson_for_student_group
        ));
    OpenApiRouter::new().merge(dont_need_permissions)
}

#[utoipa::path(post, path = "/", request_body = NewStudentGroup)]
async fn create_student_group(
    State(postgres_pool): State<PostgresPool>,
    Json(new_student_group): Json<NewStudentGroup>,
) -> Result<Json<StudentGroup>, AppError> {
    info!("Creating new student group");
    let new_student_group = StudentGroupService::create(&postgres_pool, new_student_group)?;
    Ok(Json(new_student_group))
}

#[utoipa::path(post, path = "/{id}/lessons", params(("id" = i32, Path, description = "ID группы учеников для которой создаем урок")), request_body = NewLesson)]
async fn create_lesson_for_student_group(
    State(postgres_pool): State<PostgresPool>,
    Path(student_group_id): Path<i32>,
    Json(new_lesson): Json<NewLesson>,
) -> Result<Json<Lesson>, AppError> {
    info!(
        "Creating new lesson for student group with ID {}",
        student_group_id
    );
    let new_lesson = NewLesson {
        student_group_id: Some(student_group_id),
        ..new_lesson
    };
    let new_lesson = LessonService::create(&postgres_pool, new_lesson)?;
    Ok(Json(new_lesson))
}

#[utoipa::path(get, path = "/{id}", params(("id" = i32, Path, description = "ID запрашиваемой группы учеников")))]
async fn get_student_group(
    State(postgres_pool): State<PostgresPool>,
    Path(student_group_id): Path<i32>,
) -> Result<Json<StudentGroup>, AppError> {
    info!("Getting student group");
    let student_group = StudentGroupService::get(&postgres_pool, student_group_id)?;
    Ok(Json(student_group))
}

#[utoipa::path(get, path = "/{id}/lessons", params(("id" = i32, Path, description = "ID группы учеников для которой запрашиваем уроки")))]
async fn get_lessons_for_student_group(
    State(postgres_pool): State<PostgresPool>,
    Path(student_group_id): Path<i32>,
) -> Result<Json<Vec<Lesson>>, AppError> {
    info!("Getting student group lessons");
    let lessons = StudentGroupService::get_lessons(&postgres_pool, student_group_id)?;
    Ok(Json(lessons))
}

#[utoipa::path(
    put, path = "/{id}", 
    params(("id" = i32, Path, description = "ID Группы учеников которую требуется обновить")),
    request_body = UpdateStudentGroup
)]
async fn update_student_group(
    State(postgres_pool): State<PostgresPool>,
    Path(student_group_id): Path<i32>,
    Json(update_student_group): Json<UpdateStudentGroup>,
) -> Result<Json<StudentGroup>, AppError> {
    info!("Updating student group");
    let updated_student_group =
        StudentGroupService::update(&postgres_pool, student_group_id, update_student_group)?;
    Ok(Json(updated_student_group))
}

#[utoipa::path(delete, path = "/{id}", params(("id" = i32, Path, description = "ID Группы учеников которую требуется обновить")))]
async fn delete_student_group(
    State(postgres_pool): State<PostgresPool>,
    Path(student_group_id): Path<i32>,
) -> Result<Json<String>, AppError> {
    info!("Deleting student group");
    let delete_count = StudentGroupService::delete(&postgres_pool, student_group_id)?;
    if delete_count {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Student group not found".to_string()))
    }
}
