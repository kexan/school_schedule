use axum::{
    Json,
    extract::{Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    error::AppError,
    logic::services::{attendance_service::AttendanceService, lesson_service::LessonService},
    models::{
        attendance::AttendanceWithRelations,
        lesson::{LessonWithRelations, NewLesson, UpdateLesson},
    },
};

pub fn router() -> OpenApiRouter<AppState> {
    //TODO: добавить пермишены
    let dont_need_permissions = OpenApiRouter::new()
        .routes(routes!(
            create_lesson,
            get_lesson,
            update_lesson,
            delete_lesson
        ))
        .routes(routes!(get_attendances_for_lesson));
    OpenApiRouter::new().merge(dont_need_permissions)
}

#[utoipa::path(post, path = "/", request_body = NewLesson)]
async fn create_lesson(
    State(lesson_service): State<LessonService>,
    Json(new_lesson): Json<NewLesson>,
) -> Result<Json<LessonWithRelations>, AppError> {
    info!("Creating new lesson");
    let new_lesson = lesson_service.create(new_lesson)?;
    Ok(Json(new_lesson))
}

#[utoipa::path(get, path = "/{id}", params(("id" = i32, Path, description = "ID запрашиваемого урока")))]
async fn get_lesson(
    State(lesson_service): State<LessonService>,
    Path(lesson_id): Path<i32>,
) -> Result<Json<LessonWithRelations>, AppError> {
    info!("Getting lesson");
    let lesson = lesson_service.get(lesson_id)?;
    Ok(Json(lesson))
}

#[utoipa::path(get, path = "/{id}/attendances", params(("id" = i32, Path, description = "ID урока для которого запрашваются посещения")))]
async fn get_attendances_for_lesson(
    State(attendace_service): State<AttendanceService>,
    Path(lesson_id): Path<i32>,
) -> Result<Json<Vec<AttendanceWithRelations>>, AppError> {
    info!("Getting attendances for lesson");
    let attendances = attendace_service.get_by_lesson_id(lesson_id)?;
    Ok(Json(attendances))
}

#[utoipa::path(put, path = "/{id}", params(("id" = i32, Path, description = "ID Урока который требуется обновить")), request_body = UpdateLesson)]
async fn update_lesson(
    State(lesson_service): State<LessonService>,
    Path(lesson_id): Path<i32>,
    Json(update_lesson): Json<UpdateLesson>,
) -> Result<Json<LessonWithRelations>, AppError> {
    info!("Updating lesson");
    let updated_lesson = lesson_service.update(lesson_id, update_lesson)?;
    Ok(Json(updated_lesson))
}

#[utoipa::path(delete, path = "/{id}", params(("id" = i32, Path, description = "ID Урока который требуется удалить")))]
async fn delete_lesson(
    State(lesson_service): State<LessonService>,
    Path(lesson_id): Path<i32>,
) -> Result<Json<String>, AppError> {
    info!("Deleting lesson");
    let deleted = lesson_service.delete(lesson_id)?;
    if deleted {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Lesson not found".to_string()))
    }
}
