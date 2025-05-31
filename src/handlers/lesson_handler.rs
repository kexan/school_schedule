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

/// Создание нового урока
///
/// Этот эндпоинт позволяет создать новый урок в базе данных.
///
/// ### Входные данные:
/// - `student_group_id`: ID учебной группы
/// - `topic`: Название предмета (обязательное поле)
/// - `scheduled_at`: Дата проведения урока (обязательное поле)
///
/// ### Ответы:
/// - **201 Created**: Урок успешно создан. Возвращает данные созданного урока.
/// - **400 Bad Request**: Неверные входные данные (например, отсутствуют обязательные поля).
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    post,
    path = "/",
    request_body = NewLesson,
    responses(
        (status = 201, body = LessonWithRelations, description = "Урок успешно создан"),
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Lesson"
)]
async fn create_lesson(
    State(lesson_service): State<LessonService>,
    Json(new_lesson): Json<NewLesson>,
) -> Result<Json<LessonWithRelations>, AppError> {
    info!("Creating new lesson");
    let new_lesson = lesson_service.create(new_lesson)?;
    Ok(Json(new_lesson))
}

/// Получение урока по ID
///
/// Этот эндпоинт позволяет получить данные конкретного урока по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID урока (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Данные урока успешно получены.
/// - **404 Not Found**: Урок с указанным ID не найден.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID запрашиваемого урока")
    ),
    responses(
        (status = 200, body = LessonWithRelations, description = "Данные урока успешно получены"),
        (status = 404, description = "Урок не найден"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Lesson"
)]
async fn get_lesson(
    State(lesson_service): State<LessonService>,
    Path(lesson_id): Path<i32>,
) -> Result<Json<LessonWithRelations>, AppError> {
    info!("Getting lesson");
    let lesson = lesson_service.get(lesson_id)?;
    Ok(Json(lesson))
}

/// Получение всех посещений для урока
///
/// Этот эндпоинт позволяет получить список всех посещений, связанных с конкретным уроком.
///
/// ### Параметры:
/// - `id`: ID урока (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Список посещений успешно получен.
/// - **404 Not Found**: Урок с указанным ID не найден.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    get,
    path = "/{id}/attendances",
    params(
        ("id" = i32, Path, description = "ID урока для которого запрашиваются посещения")
    ),
    responses(
        (status = 200, body = Vec<AttendanceWithRelations>, description = "Список посещений успешно получен"),
        (status = 404, description = "Урок не найден"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Lesson"
)]
async fn get_attendances_for_lesson(
    State(attendace_service): State<AttendanceService>,
    Path(lesson_id): Path<i32>,
) -> Result<Json<Vec<AttendanceWithRelations>>, AppError> {
    info!("Getting attendances for lesson");
    let attendances = attendace_service.get_by_lesson_id(lesson_id)?;
    Ok(Json(attendances))
}

/// Обновление существующего урока
///
/// Этот эндпоинт позволяет обновить данные урока по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID урока (обязательный путь)
///
/// ### Входные данные:
/// - `student_group_id`: ID учебной группы
/// - `topic`: Название предмета
/// - `scheduled_at`: Дата проведения урока
///
/// ### Ответы:
/// - **200 OK**: Данные урока успешно обновлены.
/// - **404 Not Found**: Урок с указанным ID не найден.
/// - **400 Bad Request**: Неверные входные данные.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    put,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID урока который требуется обновить")
    ),
    request_body = UpdateLesson,
    responses(
        (status = 200, body = LessonWithRelations, description = "Данные урока успешно обновлены"),
        (status = 404, description = "Урок не найден"),
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Lesson"
)]
async fn update_lesson(
    State(lesson_service): State<LessonService>,
    Path(lesson_id): Path<i32>,
    Json(update_lesson): Json<UpdateLesson>,
) -> Result<Json<LessonWithRelations>, AppError> {
    info!("Updating lesson");
    let updated_lesson = lesson_service.update(lesson_id, update_lesson)?;
    Ok(Json(updated_lesson))
}

/// Удаление урока
///
/// Этот эндпоинт позволяет удалить существующий урок по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID урока (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Урок успешно удален.
/// - **404 Not Found**: Урок с указанным ID не найден.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID урока который требуется удалить")
    ),
    responses(
        (status = 200, body = String, description = "Урок успешно удален"),
        (status = 404, description = "Урок не найден"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Lesson"
)]
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
