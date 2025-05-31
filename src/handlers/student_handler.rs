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

/// Создание нового ученика
///
/// Этот эндпоинт позволяет создать нового ученика в базе данных.
///
/// ### Входные данные:
/// - `name`: Имя ученика (обязательное поле)
/// - `birth_date`: Дата рождения (обязательное поле, формат: YYYY-MM-DD)
/// - `parent_id`: ID родителя (необязательное поле)
/// - `student_group_id`: ID учебной группы (необязательное поле)
///
/// ### Ответы:
/// - **201 Created**: Ученик успешно создан. Возвращает данные ученика с родителем и группой.
/// - **400 Bad Request**: Неверные входные данные (например, отсутствуют обязательные поля).
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    post,
    path = "/",
    request_body = NewStudent,
    responses(
        (status = 201, body = StudentWithRelations, description = "Ученик успешно создан"),
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Student"
)]
async fn create_student(
    State(student_service): State<StudentService>,
    Json(new_student): Json<NewStudent>,
) -> Result<Json<StudentWithRelations>, AppError> {
    info!("Creating new student");
    let new_student = student_service.create(new_student)?;
    Ok(Json(new_student))
}

/// Получение ученика по ID
///
/// Этот эндпоинт позволяет получить данные конкретного ученика по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID ученика (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Данные ученика успешно получены.
/// - **404 Not Found**: Ученик с указанным ID не найден.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID запрашиваемого ученика")
    ),
    responses(
        (status = 200, body = StudentWithRelations, description = "Данные ученика успешно получены"),
        (status = 404, description = "Ученик не найден"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Student"
)]
async fn get_student(
    State(student_service): State<StudentService>,
    Path(student_id): Path<i32>,
) -> Result<Json<StudentWithRelations>, AppError> {
    info!("Getting student");
    let student = student_service.get(student_id)?;
    Ok(Json(student))
}

/// Обновление существующего ученика
///
/// Этот эндпоинт позволяет обновить данные ученика по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID ученика (обязательный путь)
///
/// ### Входные данные:
/// - `name`: Новое имя ученика (необязательное поле)
/// - `birth_date`: Новая дата рождения (необязательное поле, формат: YYYY-MM-DD)
/// - `parent_id`: Новый ID родителя (необязательное поле)
/// - `student_group_id`: Новый ID учебной группы (необязательное поле)
///
/// ### Ответы:
/// - **200 OK**: Данные ученика успешно обновлены.
/// - **404 Not Found**: Ученик с указанным ID не найден.
/// - **400 Bad Request**: Неверные входные данные.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    put,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID ученика которого требуется обновить")
    ),
    request_body = UpdateStudent,
    responses(
        (status = 200, body = StudentWithRelations, description = "Данные ученика успешно обновлены"),
        (status = 404, description = "Ученик не найден"),
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Student"
)]
async fn update_student(
    State(student_service): State<StudentService>,
    Path(student_id): Path<i32>,
    Json(update_student): Json<UpdateStudent>,
) -> Result<Json<StudentWithRelations>, AppError> {
    info!("Updating student");
    let updated_student = student_service.update(student_id, update_student)?;
    Ok(Json(updated_student))
}

/// Удаление ученика
///
/// Этот эндпоинт позволяет удалить существующего ученика по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID ученика (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Ученик успешно удален.
/// - **404 Not Found**: Ученик с указанным ID не найден.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID ученика которого требуется удалить")
    ),
    responses(
        (status = 200, body = String, description = "Ученик успешно удален"),
        (status = 404, description = "Ученик не найден"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Student"
)]
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
