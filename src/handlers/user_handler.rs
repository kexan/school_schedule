use axum::{
    Json,
    extract::{Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    error::AppError,
    logic::services::user_service::UserService,
    models::user::{RawNewUser, UpdateUser, User},
};

pub fn router() -> OpenApiRouter<AppState> {
    let dont_need_permissions =
        OpenApiRouter::new().routes(routes!(create_user, get_user, update_user, delete_user));
    OpenApiRouter::new().merge(dont_need_permissions)
}

/// Создание нового пользователя
///
/// Этот эндпоинт позволяет зарегистрировать нового пользователя в системе.
///
/// ### Входные данные:
/// - `username`: Имя пользователя (обязательное поле)
/// - `password`: Пароль (обязательное поле)
/// - `full_name`: Полное имя пользователя (необязательное поле)
///
/// ### Ответы:
/// - **201 Created**: Пользователь успешно создан. Возвращает данные пользователя.
/// - **400 Bad Request**: Неверные входные данные (например, отсутствуют обязательные поля).
/// - **500 Internal Server Error**: Ошибка сервера при регистрации.
#[utoipa::path(
    post,
    path = "/",
    request_body = RawNewUser,
    responses(
        (status = 200, body = User, description = "Пользователь успешно создан"),
        (status = 400, description = "Неверные данные"),
        (status = 500, description = "Ошибка сервера")
    ),
    tag = "User"
)]
#[axum::debug_handler]
async fn create_user(
    State(user_service): State<UserService>,
    Json(new_user): Json<RawNewUser>,
) -> Result<Json<User>, AppError> {
    info!("Creating new user");
    let new_user = user_service.create(new_user)?;
    Ok(Json(new_user))
}

/// Получение пользователя по ID
///
/// Этот эндпоинт позволяет получить данные конкретного пользователя по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID пользователя (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Данные пользователя успешно получены.
/// - **404 Not Found**: Пользователь с указанным ID не найден.
/// - **500 Internal Server Error**: Ошибка сервера при получении данных.
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID запрашиваемого пользователя")
    ),
    responses(
        (status = 200, body = User, description = "Данные пользователя успешно получены"),
        (status = 404, description = "Пользователь не найден"),
        (status = 500, description = "Ошибка сервера")
    ),
    tag = "User"
)]
async fn get_user(
    State(user_service): State<UserService>,
    Path(user_id): Path<i32>,
) -> Result<Json<User>, AppError> {
    info!("Getting user by id");
    let user = user_service.get(user_id)?;
    Ok(Json(user))
}

/// Обновление существующего пользователя
///
/// Этот эндпоинт позволяет обновить данные пользователя по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID пользователя (обязательный путь)
///
/// ### Входные данные:
/// - `username`: Новое имя пользователя (необязательное поле)
/// - `role`: Новая роль пользователя (необязательное поле)
/// - `full_name`: Новое полное имя пользователя (необязательное поле)
///
/// ### Ответы:
/// - **200 OK**: Данные пользователя успешно обновлены.
/// - **404 Not Found**: Пользователь с указанным ID не найден.
/// - **400 Bad Request**: Неверные входные данные.
/// - **500 Internal Server Error**: Ошибка сервера при обновлении.
#[utoipa::path(
    put,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID пользователя который требуется обновить")
    ),
    request_body = UpdateUser,
    responses(
        (status = 200, body = User, description = "Данные пользователя успешно обновлены"),
        (status = 404, description = "Пользователь не найден"),
        (status = 400, description = "Неверные данные"),
        (status = 500, description = "Ошибка сервера")
    ),
    tag = "User"
)]
async fn update_user(
    State(user_service): State<UserService>,
    Path(user_id): Path<i32>,
    Json(updated_user): Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    info!("Updating user by id");
    let user = user_service.update(user_id, updated_user)?;
    Ok(Json(user))
}

/// Удаление пользователя
///
/// Этот эндпоинт позволяет удалить существующего пользователя по его идентификатору.
///
/// ### Параметры:
/// - `id`: ID пользователя (обязательный путь)
///
/// ### Ответы:
/// - **200 OK**: Пользователь успешно удален.
/// - **404 Not Found**: Пользователь с указанным ID не найден.
/// - **500 Internal Server Error**: Ошибка сервера при удалении.
#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID пользователя который требуется удалить")
    ),
    responses(
        (status = 200, body = String, description = "Пользователь успешно удален"),
        (status = 404, description = "Пользователь не найден"),
        (status = 500, description = "Ошибка сервера")
    ),
    tag = "User"
)]
async fn delete_user(
    State(user_service): State<UserService>,
    Path(user_id): Path<i32>,
) -> Result<Json<String>, AppError> {
    info!("Deleting user by id");
    let result = user_service.delete(user_id);
    if result.is_ok() {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("User not found".to_string()))
    }
}
