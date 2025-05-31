use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    AppState,
    error::AppError,
    logic::services::{document_service::DocumentService, teacher_service::TeacherService},
    models::{
        document::{Document, DocumentFileForm},
        teacher::{NewTeacher, Teacher, UpdateTeacher},
    },
};

pub fn router() -> OpenApiRouter<AppState> {
    let dont_need_permissions = OpenApiRouter::new()
        .routes(routes!(
            create_teacher,
            get_teacher,
            update_teacher,
            delete_teacher
        ))
        .routes(routes!(
            get_teacher_documents,
            upload_document,
            delete_document
        ));
    OpenApiRouter::new().merge(dont_need_permissions)
}

/// Создание нового преподавателя
///
/// Этот эндпоинт позволяет создать нового преподавателя в базе данных.
///
/// ### Входные данные:
/// - `name`: Имя преподавателя (обязательное поле)
///
/// ### Ответы:
/// - **201 Created**: Преподаватель успешно создан. Возвращает данные созданного преподавателя.
/// - **400 Bad Request**: Неверные входные данные (например, отсутствует обязательное поле).
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    post,
    path = "/",
    request_body = NewTeacher,
    responses(
        (status = 201, body = Teacher, description = "Преподаватель успешно создан"),
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Teacher"
)]
async fn create_teacher(
    State(teacher_service): State<TeacherService>,
    Json(new_teacher): Json<NewTeacher>,
) -> Result<Json<Teacher>, AppError> {
    info!("Creating new teacher");
    let new_teacher = teacher_service.create(new_teacher)?;
    Ok(Json(new_teacher))
}

/// Загрузка документа для преподавателя
///
/// Этот эндпоинт позволяет загрузить документ для конкретного преподавателя.
///
/// ### Параметры:
/// - `id`: ID преподавателя (обязательный путь)
///
/// ### Входные данные:
/// - `document`: Файл документа в формате multipart/form-data
///
/// ### Ответы:
/// - **201 Created**: Документ успешно загружен. Возвращает данные документа.
/// - **400 Bad Request**: Неверные входные данные или отсутствует файл.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    post,
    path = "/{id}/documents",
    params(
        ("id" = i32, Path, description = "ID преподавателя к которому загружаем документ")
    ),
    request_body(
        content_type = "multipart/form-data",
        content = DocumentFileForm,
        description = "Загружаемый документ"
    ),
    responses(
        (status = 201, body = Document, description = "Документ успешно загружен"),
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Teacher"
)]
async fn upload_document(
    State(document_service): State<DocumentService>,
    Path(teacher_id): Path<i32>,
    multipart: Multipart,
) -> Result<Json<Document>, AppError> {
    info!("Uploading document to teacher with ID {}", teacher_id);
    let document = document_service.create(multipart, teacher_id).await?;
    Ok(Json(document))
}

/// Получение преподавателя по ID
///
/// Этот эндпоинт позволяет получить данные конкретного преподавателя по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID преподавателя (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Данные преподавателя успешно получены.
/// - **404 Not Found**: Преподаватель с указанным ID не найден.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID запрашиваемого преподавателя")
    ),
    responses(
        (status = 200, body = Teacher, description = "Данные преподавателя успешно получены"),
        (status = 404, description = "Преподаватель не найден"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Teacher"
)]
async fn get_teacher(
    State(teacher_service): State<TeacherService>,
    Path(teacher_id): Path<i32>,
) -> Result<Json<Teacher>, AppError> {
    info!("Getting teacher with ID {}", teacher_id);
    let teacher = teacher_service.get(teacher_id)?;
    Ok(Json(teacher))
}

/// Получение всех документов преподавателя
///
/// Этот эндпоинт позволяет получить список всех документов, связанных с конкретным преподавателем.
///
/// ### Параметры:
/// - `id`: ID преподавателя (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Список документов успешно получен.
/// - **404 Not Found**: Преподаватель с указанным ID не найден.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    get,
    path = "/{id}/documents",
    params(
        ("id" = i32, Path, description = "ID преподавателя у которого запрашиваются документы")
    ),
    responses(
        (status = 200, body = Vec<Document>, description = "Список документов успешно получен"),
        (status = 404, description = "Преподаватель не найден"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Teacher"
)]
async fn get_teacher_documents(
    State(document_service): State<DocumentService>,
    Path(teacher_id): Path<i32>,
) -> Result<Json<Vec<Document>>, AppError> {
    info!("Getting documents for teacher with ID {}", teacher_id);
    let documents = document_service.get_by_teacher_id(teacher_id)?;
    Ok(Json(documents))
}

/// Обновление существующего преподавателя
///
/// Этот эндпоинт позволяет обновить данные преподавателя по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID преподавателя (обязательный путь)
///
/// ### Входные данные:
/// - `name`: Новое имя преподавателя (необязательное поле)
///
/// ### Ответы:
/// - **200 OK**: Данные преподавателя успешно обновлены.
/// - **404 Not Found**: Преподаватель с указанным ID не найден.
/// - **400 Bad Request**: Неверные входные данные.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    put,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID преподавателя который требуется обновить")
    ),
    request_body = UpdateTeacher,
    responses(
        (status = 200, body = Teacher, description = "Данные преподавателя успешно обновлены"),
        (status = 404, description = "Преподаватель не найден"),
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Teacher"
)]
async fn update_teacher(
    State(teacher_service): State<TeacherService>,
    Path(teacher_id): Path<i32>,
    Json(update_teacher): Json<UpdateTeacher>,
) -> Result<Json<Teacher>, AppError> {
    info!("Updating teacher with ID {}", teacher_id);
    let updated_teacher = teacher_service.update(teacher_id, update_teacher)?;
    Ok(Json(updated_teacher))
}

/// Удаление преподавателя
///
/// Этот эндпоинт позволяет удалить существующего преподавателя по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID преподавателя (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Преподаватель успешно удален.
/// - **404 Not Found**: Преподаватель с указанным ID не найден.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID преподавателя который требуется удалить")
    ),
    responses(
        (status = 200, body = String, description = "Преподаватель успешно удален"),
        (status = 404, description = "Преподаватель не найден"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Teacher"
)]
async fn delete_teacher(
    State(teacher_service): State<TeacherService>,
    Path(teacher_id): Path<i32>,
) -> Result<Json<String>, AppError> {
    info!("Deleting teacher with ID {}", teacher_id);
    let deleted = teacher_service.delete(teacher_id)?;
    if deleted {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Teacher not found".to_string()))
    }
}

/// Удаление документа преподавателя
///
/// Этот эндпоинт позволяет удалить конкретный документ преподавателя по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID преподавателя (обязательный путь)
/// - `document_id`: ID документа (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Документ успешно удален.
/// - **404 Not Found**: Документ или преподаватель с указанными ID не найдены.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    delete,
    path = "/{id}/documents/{document_id}",
    params(
        ("id" = i32, Path, description = "ID преподавателя"),
        ("document_id" = Uuid, Path, description = "ID документа который нужно удалить")
    ),
    responses(
        (status = 200, body = String, description = "Документ удален"),
        (status = 404, description = "Документ или преподаватель не найдены"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Teacher"
)]
async fn delete_document(
    State(document_service): State<DocumentService>,
    Path((teacher_id, document_id)): Path<(i32, Uuid)>,
) -> Result<Json<String>, AppError> {
    info!(
        "Deleting document {} from teacher with ID {}",
        document_id, teacher_id
    );
    let deleted = document_service.delete(document_id)?;
    if deleted {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Document not found".to_string()))
    }
}
