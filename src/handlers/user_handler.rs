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
    models::user::{UpdateUser, User},
};

//TODO: Документация
pub fn router() -> OpenApiRouter<AppState> {
    let dont_need_permissions =
        OpenApiRouter::new().routes(routes!(get_user, update_user, delete_user));
    OpenApiRouter::new().merge(dont_need_permissions)
}

//TODO: нельзя использовать и жсон и форм
// #[utoipa::path(post,
//     path = "/",
//     responses(
//         (status = 200, description = "Пользователь успешно создан"),
//         (status = 400, description = "Неверные данные")
//     ),
//     tag = "User"
// )]
// #[axum::debug_handler]
// async fn create_user(
//     State(user_service): State<UserService>,
//     Json(new_user): Json<NewUser>,
//     Form(creds): Form<Credentials>,
// ) -> Result<Json<User>, AppError> {
//     info!("Creating new user");
//     let new_user = user_service.create(new_user, creds)?;
//     Ok(Json(new_user))
// }

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID запрашиваемого пользователя")
    ),
    responses(
        (status = 200, body = User, description = "Данные пользователя успешно получены"),
        (status = 404, description = "Пользователь не найден"),
        (status = 500, description = "Внутренняя ошибка сервера")
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
        (status = 400, description = "Неверные входные данные"),
        (status = 500, description = "Внутренняя ошибка сервера")
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

#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID пользователя который требуется удалить")
    ),
    responses(
        (status = 200, body = String, description = "Пользователь успешно удален"),
        (status = 404, description = "Пользователь не найден"),
        (status = 500, description = "Внутренняя ошибка сервера")
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
