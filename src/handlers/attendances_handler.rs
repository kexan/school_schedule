use axum::{
    Json,
    extract::{Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    error::AppError,
    logic::services::attendance_service::AttendanceService,
    models::attendance::{AttendanceWithRelations, NewAttendance, UpdateAttendance},
};

pub fn router() -> OpenApiRouter<AppState> {
    let dont_need_permissions = OpenApiRouter::new()
        .routes(routes!(
            create_attendance,
            get_attendance,
            update_attendance,
            delete_attendance,
        ))
        .routes(routes!(get_attendances_by_lesson));
    OpenApiRouter::new().merge(dont_need_permissions)
}

/// Создание нового посещения
///
/// Этот эндпоинт позволяет создать новое посещение в базе данных.
///
/// ### Входные данные:
/// - `student_id`: ID студента (обязательное поле)
/// - `lesson_id`: ID урока (обязательное поле)
/// - `is_present`: Отметка о том, что студент присутствовал (обязательное поле, по умолчанию
/// false)
/// - `skip_reason`: Причина пропуска
///
/// ### Ответы:
/// - **201 Created**: Посещение успешно создано. Возвращает данные созданного посещения.
/// - **400 Bad Request**: Неверные входные данные (например, отсутствуют обязательные поля).
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    post,
    path = "/",
    request_body = NewAttendance,
    responses(
        (status = 201, body = AttendanceWithRelations, description = "Посещение успешно создано"),
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Attendance"
)]
async fn create_attendance(
    State(attendance_service): State<AttendanceService>,
    Json(new_attendance): Json<NewAttendance>,
) -> Result<Json<AttendanceWithRelations>, AppError> {
    info!("Creating new attendance");
    let created_attendance = attendance_service.create(new_attendance)?;
    Ok(Json(created_attendance))
}

/// Получение посещения по ID
///
/// Этот эндпоинт позволяет получить данные конкретного посещения по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID посещения (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Данные посещения успешно получены.
/// - **404 Not Found**: Посещение с указанным ID не найдено.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID запрашиваемого посещения")
    ),
    responses(
        (status = 200, body = AttendanceWithRelations, description = "Данные посещения успешно получены"),
        (status = 404, description = "Посещение не найдено"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Attendance"
)]
async fn get_attendance(
    State(attendance_service): State<AttendanceService>,
    Path(attendance_id): Path<i32>,
) -> Result<Json<AttendanceWithRelations>, AppError> {
    info!("Getting attendance");
    let attendance = attendance_service.get(attendance_id)?;
    Ok(Json(attendance))
}

/// Получение всех посещений по ID урока
///
/// Этот эндпоинт позволяет получить список всех посещений, связанных с конкретным уроком.
///
/// ### Параметры:
/// - `lesson_id`: ID урока (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Список посещений успешно получен.
/// - **404 Not Found**: Урок с указанным ID не найден.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    get,
    path = "/lesson/{lesson_id}",
    params(
        ("lesson_id" = i32, Path, description = "ID урока для которого запрашиваются посещения")
    ),
    responses(
        (status = 200, body = Vec<AttendanceWithRelations>, description = "Список посещений успешно получен"),
        (status = 404, description = "Урок не найден"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Attendance"
)]
async fn get_attendances_by_lesson(
    State(attendance_service): State<AttendanceService>,
    Path(lesson_id): Path<i32>,
) -> Result<Json<Vec<AttendanceWithRelations>>, AppError> {
    info!("Getting attendances by lesson");
    let attendances = attendance_service.get_by_lesson_id(lesson_id)?;
    Ok(Json(attendances))
}

/// Обновление существующего посещения
///
/// Этот эндпоинт позволяет обновить данные посещения по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID посещения (обязательный путь)
///
/// ### Входные данные:
/// - `is_present`: Отметка о том, что студент присутствовал
/// - `skip_reason`: Причина пропуска
///
/// ### Ответы:
/// - **200 OK**: Данные посещения успешно обновлены.
/// - **404 Not Found**: Посещение с указанным ID не найдено.
/// - **400 Bad Request**: Неверные входные данные.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    put,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID посещения которое требуется обновить")
    ),
    request_body = UpdateAttendance,
    responses(
        (status = 200, body = AttendanceWithRelations, description = "Данные посещения успешно обновлены"),
        (status = 404, description = "Посещение не найдено"),
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Attendance"
)]
async fn update_attendance(
    State(attendance_service): State<AttendanceService>,
    Path(attendance_id): Path<i32>,
    Json(update_attendance): Json<UpdateAttendance>,
) -> Result<Json<AttendanceWithRelations>, AppError> {
    info!("Updating attendance");
    let updated_attendance = attendance_service.update(attendance_id, update_attendance)?;
    Ok(Json(updated_attendance))
}

/// Удаление посещения
///
/// Этот эндпоинт позволяет удалить существующее посещение по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID посещения (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Посещение успешно удалено.
/// - **404 Not Found**: Посещение с указанным ID не найдено.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID посещения которое требуется удалить")
    ),
    responses(
        (status = 200, body = String, description = "Посещение успешно удалено"),
        (status = 404, description = "Посещение не найдено"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Attendance"
)]
async fn delete_attendance(
    State(attendance_service): State<AttendanceService>,
    Path(attendance_id): Path<i32>,
) -> Result<Json<String>, AppError> {
    info!("Deleting attendance");
    let deleted = attendance_service.delete(attendance_id)?;
    if deleted {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Attendance not found".to_string()))
    }
}
