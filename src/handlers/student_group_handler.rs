use axum::{
    Json,
    extract::{Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    error::AppError,
    logic::services::{lesson_service::LessonService, student_group_service::StudentGroupService},
    models::{
        lesson::{Lesson, LessonWithRelations, NewLesson},
        student_group::{NewStudentGroup, StudentGroupWithRelations, UpdateStudentGroup},
    },
};

pub fn router() -> OpenApiRouter<AppState> {
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
    State(student_group_service): State<StudentGroupService>,
    Json(new_student_group): Json<NewStudentGroup>,
) -> Result<Json<StudentGroupWithRelations>, AppError> {
    info!("Creating new student group");
    let new_student_group = student_group_service.create(new_student_group)?;
    Ok(Json(new_student_group))
}

#[utoipa::path(
    post,
    path = "/{id}/lessons",
    params(("id" = i32, Path, description = "ID группы учеников для которой создаем урок")),
    request_body = NewLesson
)]
async fn create_lesson_for_student_group(
    State(lesson_service): State<LessonService>,
    Path(student_group_id): Path<i32>,
    Json(mut new_lesson): Json<NewLesson>,
) -> Result<Json<LessonWithRelations>, AppError> {
    info!(
        "Creating new lesson for student group with ID {}",
        student_group_id
    );
    new_lesson.student_group_id = Some(student_group_id);
    let new_lesson = lesson_service.create(new_lesson)?;
    Ok(Json(new_lesson))
}

#[utoipa::path(get, path = "/{id}", params(("id" = i32, Path, description = "ID запрашиваемой группы учеников")))]
async fn get_student_group(
    State(student_group_service): State<StudentGroupService>,
    Path(student_group_id): Path<i32>,
) -> Result<Json<StudentGroupWithRelations>, AppError> {
    info!("Getting student group");
    let student_group = student_group_service.get(student_group_id)?;
    Ok(Json(student_group))
}

#[utoipa::path(
    get,
    path = "/{id}/lessons",
    params(("id" = i32, Path, description = "ID группы учеников для которой запрашиваем уроки"))
)]
async fn get_lessons_for_student_group(
    State(lessons_service): State<LessonService>,
    Path(student_group_id): Path<i32>,
) -> Result<Json<Vec<Lesson>>, AppError> {
    info!("Getting lessons for student group");
    let lessons = lessons_service.get_lessons_by_group_id(student_group_id)?;
    Ok(Json(lessons))
}

#[utoipa::path(
    put,
    path = "/{id}",
    params(("id" = i32, Path, description = "ID Группы учеников которую требуется обновить")),
    request_body = UpdateStudentGroup
)]
async fn update_student_group(
    State(student_group_service): State<StudentGroupService>,
    Path(student_group_id): Path<i32>,
    Json(update_student_group): Json<UpdateStudentGroup>,
) -> Result<Json<StudentGroupWithRelations>, AppError> {
    info!("Updating student group");
    let updated_student_group =
        student_group_service.update(student_group_id, update_student_group)?;
    Ok(Json(updated_student_group))
}

#[utoipa::path(delete, path = "/{id}", params(("id" = i32, Path, description = "ID Группы учеников которую требуется удалить")))]
async fn delete_student_group(
    State(student_group_service): State<StudentGroupService>,
    Path(student_group_id): Path<i32>,
) -> Result<Json<String>, AppError> {
    info!("Deleting student group");
    let deleted = student_group_service.delete(student_group_id)?;
    if deleted {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Student group not found".to_string()))
    }
}
