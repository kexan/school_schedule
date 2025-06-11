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

//TODO: Документация
pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(login))
        .routes(routes!(logout))
}

#[utoipa::path(post, path = "/login", responses((status = 200, description = "Пользователь успешно авторизован"), (status = 401, description = "Неверные данные")), tag = "Auth")]
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

#[utoipa::path(post, path = "/logout", responses((status = 200, description = "Пользователь успешно разлогинен")), tag = "Auth")]
async fn logout(mut auth_session: AuthSession<AuthBackend>) -> Result<StatusCode, AppError> {
    match auth_session.logout().await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(AppError::InternalServerError(e.to_string())),
    }
}
