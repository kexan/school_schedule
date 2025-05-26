use axum::{
    Json,
    extract::{Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    db::PostgresPool,
    error::AppError,
    logic::services::lesson_service::LessonService,
    models::{
        attendance::Attendance,
        lesson::{LessonWithRelations, NewLesson, UpdateLesson},
    },
};

pub fn router() -> OpenApiRouter<PostgresPool> {
    //TODO: добавить пермишены
    let dont_need_permissions = OpenApiRouter::new()
        .routes(routes!(
            create_lesson,
            get_lesson,
            update_lesson,
            delete_lesson
        ))
        .routes(routes!(get_atttendances_for_lesson));
    OpenApiRouter::new().merge(dont_need_permissions)
}

#[utoipa::path(post, path = "/", request_body = NewLesson)]
async fn create_lesson(
    State(services): State,
    Json(new_lesson): Json<NewLesson>,
) -> Result<Json<LessonWithRelations>, AppError> {
    info!("Creating new lesson");
    let new_lesson = LessonService::create(&postgres_pool, new_lesson)?;
    Ok(Json(new_lesson))
}

#[utoipa::path(get, path = "/{id}", params(("id" = i32, Path, description = "ID запрашиваемого урока")))]
async fn get_lesson(
    State(postgres_pool): State<PostgresPool>,
    Path(lesson_id): Path<i32>,
) -> Result<Json<LessonWithRelations>, AppError> {
    info!("Getting lesson");
    let lesson = LessonService::get(&postgres_pool, lesson_id)?;
    Ok(Json(lesson))
}

#[utoipa::path(get, path = "/{id}/attendances", params(("id" = i32, Path, description = "ID урока для которого запрашваются посещения")))]
async fn get_atttendances_for_lesson(
    State(postgres_pool): State<PostgresPool>,
    Path(lesson_id): Path<i32>,
) -> Result<Json<Vec<Attendance>>, AppError> {
    info!("Getting lesson");
    let attendances = LessonService::get_attendances(&postgres_pool, lesson_id)?;
    Ok(Json(attendances))
}

#[utoipa::path(put, path = "/{id}", params(("id" = i32, Path, description = "ID Урока который требуется обновить")), request_body = UpdateLesson)]
async fn update_lesson(
    State(postgres_pool): State<PostgresPool>,
    Path(lesson_id): Path<i32>,
    Json(update_lesson): Json<UpdateLesson>,
) -> Result<Json<LessonWithRelations>, AppError> {
    info!("Updating lesson");
    let updated_lesson = LessonService::update(&postgres_pool, lesson_id, update_lesson)?;
    Ok(Json(updated_lesson))
}

#[utoipa::path(delete, path = "/{id}", params(("id" = i32, Path, description = "ID Урока который требуется удалить")))]
async fn delete_lesson(
    State(postgres_pool): State<PostgresPool>,
    Path(lesson_id): Path<i32>,
) -> Result<Json<String>, AppError> {
    info!("Deleting lesson");
    let delete_count = LessonService::delete(&postgres_pool, lesson_id)?;
    if delete_count {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Lesson not found".to_string()))
    }
}
