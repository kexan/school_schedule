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

#[utoipa::path(post, path = "/", request_body = NewParent)]
async fn create_parent(
    State(parent_service): State<ParentService>,
    Json(new_parent): Json<NewParent>,
) -> Result<Json<Parent>, AppError> {
    info!("Creating new parent");
    let new_parent = parent_service.create(new_parent)?;
    Ok(Json(new_parent))
}

#[utoipa::path(get, path = "/{id}", params(("id" = i32, Path, description = "ID запрашиваемого родителя")))]
async fn get_parent(
    State(parent_service): State<ParentService>,
    Path(parent_id): Path<i32>,
) -> Result<Json<Parent>, AppError> {
    info!("Getting parent");
    let parent = parent_service.get(parent_id)?;
    Ok(Json(parent))
}

#[utoipa::path(put, path = "/{id}", params(("id" = i32, Path, description = "ID Родителя которого требуется обновить")), request_body = UpdateParent)]
async fn update_parent(
    State(parent_service): State<ParentService>,
    Path(parent_id): Path<i32>,
    Json(update_parent): Json<UpdateParent>,
) -> Result<Json<Parent>, AppError> {
    info!("Updating parent");
    let updated_parent = parent_service.update(parent_id, update_parent)?;
    Ok(Json(updated_parent))
}

#[utoipa::path(delete, path = "/{id}", params(("id" = i32, Path, description = "ID Родителя которого требуется удалить")))]
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
