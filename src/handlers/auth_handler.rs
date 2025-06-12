use axum::{Form, Json, http::StatusCode};
use axum_login::AuthSession;
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    auth::backend::AuthBackend,
    error::AppError,
    models::user::{Credentials, User},
};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(login))
        .routes(routes!(logout))
}

/// Вход пользователя в систему
///
/// Этот эндпоинт позволяет авторизовать пользователя по его логину и паролю.
///
/// ### Входные данные:
/// - `username`: Имя пользователя (обязательное поле)
/// - `password`: Пароль (обязательное поле)
///
/// ### Ответы:
/// - **200 OK**: Пользователь успешно авторизован. Возвращает объект пользователя.
/// - **401 Unauthorized**: Неверный логин или пароль.
/// - **500 Internal Server Error**: Ошибка сервера при аутентификации.
#[utoipa::path(
    post,
    path = "/login",
    request_body = Credentials,
    responses(
        (status = 200, body = User, description = "Пользователь успешно авторизован"),
        (status = 401, description = "Неверные данные"),
        (status = 500, description = "Ошибка сервера")
    ),
    tag = "Auth"
)]
async fn login(
    mut auth_session: AuthSession<AuthBackend>,
    Form(creds): Form<Credentials>,
) -> Result<Json<User>, AppError> {
    info!(
        "Attempting to log in user with username: {}",
        creds.username
    );
    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(AppError::Unauthorized("User not found".to_string()));
        }
        Err(e) => return Err(AppError::Unauthorized(e.to_string())),
    };

    match auth_session.login(&user).await {
        Ok(_) => Ok(Json(user)),
        Err(e) => Err(AppError::Unauthorized(e.to_string())),
    }
}

/// Выход пользователя из системы
///
/// Этот эндпоинт завершает сессию текущего пользователя.
///
/// ### Ответы:
/// - **200 OK**: Пользователь успешно разлогинен.
/// - **500 Internal Server Error**: Ошибка сервера при выходе.
#[utoipa::path(
    post,
    path = "/logout",
    responses(
        (status = 200, description = "Пользователь успешно разлогинен"),
        (status = 500, description = "Ошибка сервера")
    ),
    tag = "Auth"
)]
async fn logout(mut auth_session: AuthSession<AuthBackend>) -> Result<StatusCode, AppError> {
    match auth_session.logout().await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(AppError::InternalServerError(e.to_string())),
    }
}
