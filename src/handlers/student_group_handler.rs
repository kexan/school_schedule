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

/// Создание новой учебной группы
///
/// Этот эндпоинт позволяет создать новую учебную группу в базе данных.
///
/// ### Входные данные:
/// - `direction`: Направление обучения (необязательное поле)
/// - `free_spots`: Количество свободных мест (обязательное поле)
/// - `teacher_id`: ID преподавателя (необязательное поле)
///
/// ### Ответы:
/// - **201 Created**: Группа успешно создана. Возвращает данные созданной группы с преподавателем.
/// - **400 Bad Request**: Неверные входные данные (например, отсутствует обязательное поле).
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    post,
    path = "/",
    request_body = NewStudentGroup,
    responses(
        (status = 201, body = StudentGroupWithRelations, description = "Группа успешно создана"),
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "StudentGroup"
)]
async fn create_student_group(
    State(student_group_service): State<StudentGroupService>,
    Json(new_student_group): Json<NewStudentGroup>,
) -> Result<Json<StudentGroupWithRelations>, AppError> {
    info!("Creating new student group");
    let new_student_group = student_group_service.create(new_student_group)?;
    Ok(Json(new_student_group))
}

/// Создание урока для учебной группы
///
/// Этот эндпоинт позволяет создать новый урок для конкретной учебной группы.
///
/// ### Параметры:
/// - `id`: ID учебной группы (обязательный путь)
///
/// ### Входные данные:
/// - `topic`: Название предмета (обязательное поле)
/// - `scheduled_at`: Дата проведения урока (обязательное поле)
/// -  Поле `student_group_id` будет проигнорировано, даже если передано - в этом эндпоинте всегда
/// используется айди группы указанный в пути.
///
/// ### Ответы:
/// - **201 Created**: Урок успешно создан. Возвращает данные созданного урока.
/// - **400 Bad Request**: Неверные входные данные.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    post,
    path = "/{id}/lessons",
    params(
        ("id" = i32, Path, description = "ID группы учеников для которой создаем урок")
    ),
    request_body = NewLesson,
    responses(
        (status = 201, body = LessonWithRelations, description = "Урок успешно создан"),
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "StudentGroup"
)]
async fn create_lesson_for_student_group(
    State(lesson_service): State<LessonService>,
    Path(student_group_id): Path<i32>,
    Json(mut new_lesson): Json<NewLesson>,
) -> Result<Json<LessonWithRelations>, AppError> {
    info!("Creating new lesson for student group");
    new_lesson.student_group_id = Some(student_group_id);
    let new_lesson = lesson_service.create(new_lesson)?;
    Ok(Json(new_lesson))
}

/// Получение учебной группы по ID
///
/// Этот эндпоинт позволяет получить данные конкретной учебной группы по ее идентификатору.
///
/// ### Параметры:
/// - `id`: ID группы (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Данные группы успешно получены.
/// - **404 Not Found**: Группа с указанным ID не найдена.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID запрашиваемой группы учеников")
    ),
    responses(
        (status = 200, body = StudentGroupWithRelations, description = "Данные группы успешно получены"),
        (status = 404, description = "Группа не найдена"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "StudentGroup"
)]
async fn get_student_group(
    State(student_group_service): State<StudentGroupService>,
    Path(student_group_id): Path<i32>,
) -> Result<Json<StudentGroupWithRelations>, AppError> {
    info!("Getting student group");
    let student_group = student_group_service.get(student_group_id)?;
    Ok(Json(student_group))
}

/// Получение всех уроков для учебной группы
///
/// Этот эндпоинт позволяет получить список всех уроков, связанных с конкретной учебной группой.
///
/// ### Параметры:
/// - `id`: ID группы (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Список уроков успешно получен.
/// - **404 Not Found**: Группа с указанным ID не найдена.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    get,
    path = "/{id}/lessons",
    params(
        ("id" = i32, Path, description = "ID группы учеников для которой запрашиваем уроки")
    ),
    responses(
        (status = 200, body = Vec<Lesson>, description = "Список уроков успешно получен"),
        (status = 404, description = "Группа не найдена"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "StudentGroup"
)]
async fn get_lessons_for_student_group(
    State(lessons_service): State<LessonService>,
    Path(student_group_id): Path<i32>,
) -> Result<Json<Vec<Lesson>>, AppError> {
    info!("Getting lessons for student group");
    let lessons = lessons_service.get_lessons_by_group_id(student_group_id)?;
    Ok(Json(lessons))
}

/// Обновление существующей учебной группы
///
/// Этот эндпоинт позволяет обновить данные учебной группы по ее идентификатору.
///
/// ### Параметры:
/// - `id`: ID группы (обязательный путь)
///
/// ### Входные данные:
/// - `direction`: Новое направление обучения (необязательное поле)
/// - `free_spots`: Новое количество свободных мест (необязательное поле)
/// - `teacher_id`: Новый ID преподавателя (необязательное поле)
///
/// ### Ответы:
/// - **200 OK**: Данные группы успешно обновлены.
/// - **404 Not Found**: Группа с указанным ID не найдена.
/// - **400 Bad Request**: Неверные входные данные.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    put,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID группы учеников которую требуется обновить")
    ),
    request_body = UpdateStudentGroup,
    responses(
        (status = 200, body = StudentGroupWithRelations, description = "Данные группы успешно обновлены"),
        (status = 404, description = "Группа не найдена"),
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "StudentGroup"
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

/// Удаление учебной группы
///
/// Этот эндпоинт позволяет удалить существующую учебную группу по ее идентификатору.
///
/// ### Параметры:
/// - `id`: ID группы (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Группа успешно удалена.
/// - **404 Not Found**: Группа с указанным ID не найдена.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID группы учеников которую требуется удалить")
    ),
    responses(
        (status = 200, body = String, description = "Группа успешно удалена"),
        (status = 404, description = "Группа не найдена"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "StudentGroup"
)]
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
