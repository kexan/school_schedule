use axum::{
    Json,
    extract::{Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    error::AppError,
    logic::services::parent_service::ParentService,
    models::parent::{NewParent, Parent, UpdateParent},
};

pub fn router() -> OpenApiRouter<AppState> {
    let dont_need_permissions = OpenApiRouter::new().routes(routes!(
        create_parent,
        get_parent,
        update_parent,
        delete_parent
    ));
    OpenApiRouter::new().merge(dont_need_permissions)
}

/// Создание нового родителя
///
/// Этот эндпоинт позволяет создать нового родителя в базе данных.
///
/// ### Входные данные:
/// - `name`: Имя родителя (обязательное поле)
/// - `additional_info`: Дополнительная информация (необязательное поле)
///
/// ### Ответы:
/// - **201 Created**: Родитель успешно создан. Возвращает данные созданного родителя.
/// - **400 Bad Request**: Неверные входные данные (например, отсутствуют обязательные поля).
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    post,
    path = "/",
    request_body = NewParent,
    responses(
        (status = 201, body = Parent, description = "Родитель успешно создан"),
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Parent"
)]
async fn create_parent(
    State(parent_service): State<ParentService>,
    Json(new_parent): Json<NewParent>,
) -> Result<Json<Parent>, AppError> {
    info!("Creating new parent");
    let new_parent = parent_service.create(new_parent)?;
    Ok(Json(new_parent))
}

/// Получение родителя по ID
///
/// Этот эндпоинт позволяет получить данные конкретного родителя по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID родителя (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Данные родителя успешно получены.
/// - **404 Not Found**: Родитель с указанным ID не найден.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID запрашиваемого родителя")
    ),
    responses(
        (status = 200, body = Parent, description = "Данные родителя успешно получены"),
        (status = 404, description = "Родитель не найден"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Parent"
)]
async fn get_parent(
    State(parent_service): State<ParentService>,
    Path(parent_id): Path<i32>,
) -> Result<Json<Parent>, AppError> {
    info!("Getting parent");
    let parent = parent_service.get(parent_id)?;
    Ok(Json(parent))
}

/// Обновление существующего родителя
///
/// Этот эндпоинт позволяет обновить данные родителя по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID родителя (обязательный путь)
///
/// ### Входные данные:
/// - `name`: Новое имя родителя (необязательное поле)
/// - `additional_info`: Новая дополнительная информация (необязательное поле)
///
/// ### Ответы:
/// - **200 OK**: Данные родителя успешно обновлены.
/// - **404 Not Found**: Родитель с указанным ID не найден.
/// - **400 Bad Request**: Неверные входные данные.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    put,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID родителя которого требуется обновить")
    ),
    request_body = UpdateParent,
    responses(
        (status = 200, body = Parent, description = "Данные родителя успешно обновлены"),
        (status = 404, description = "Родитель не найден"),
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Parent"
)]
async fn update_parent(
    State(parent_service): State<ParentService>,
    Path(parent_id): Path<i32>,
    Json(update_parent): Json<UpdateParent>,
) -> Result<Json<Parent>, AppError> {
    info!("Updating parent");
    let updated_parent = parent_service.update(parent_id, update_parent)?;
    Ok(Json(updated_parent))
}

/// Удаление родителя
///
/// Этот эндпоинт позволяет удалить существующего родителя по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID родителя (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Родитель успешно удален.
/// - **404 Not Found**: Родитель с указанным ID не найден.
/// - **500 Internal Server Error**: Внутренняя ошибка сервера.
#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID родителя которого требуется удалить")
    ),
    responses(
        (status = 200, body = String, description = "Родитель успешно удален"),
        (status = 404, description = "Родитель не найден"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "Parent"
)]
async fn delete_parent(
    State(parent_service): State<ParentService>,
    Path(parent_id): Path<i32>,
) -> Result<Json<String>, AppError> {
    info!("Deleting parent");
    let deleted = parent_service.delete(parent_id)?;
    if deleted {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Parent not found".to_string()))
    }
}
