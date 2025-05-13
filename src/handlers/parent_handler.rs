use axum::{
    Json,
    extract::{Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    db::PostgresPool,
    error::AppError,
    logic::services::parent_service::ParentService,
    models::parent::{NewParent, Parent, UpdateParent},
};

pub fn router() -> OpenApiRouter<PostgresPool> {
    //TODO: добавить пермишены
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
    State(postgres_pool): State<PostgresPool>,
    Json(new_student): Json<NewParent>,
) -> Result<Json<Parent>, AppError> {
    info!("Creating new parent");
    let new_parent = ParentService::create(&postgres_pool, new_student)?;
    Ok(Json(new_parent))
}

#[utoipa::path(get, path = "/{id}", params(("id" = i32, Path, description = "ID запрашиваемого родителя")))]
async fn get_parent(
    State(postgres_pool): State<PostgresPool>,
    Path(student_id): Path<i32>,
) -> Result<Json<Parent>, AppError> {
    info!("Getting parent");
    let parent = ParentService::get(&postgres_pool, student_id)?;
    Ok(Json(parent))
}

#[utoipa::path(put, path = "/{id}", params(("id" = i32, Path, description = "ID Родителя которого требуется обновить")), request_body = UpdateParent)]
async fn update_parent(
    State(postgres_pool): State<PostgresPool>,
    Path(parent_id): Path<i32>,
    Json(update_parent): Json<UpdateParent>,
) -> Result<Json<Parent>, AppError> {
    info!("Updating parent");
    let updated_parent = ParentService::update(&postgres_pool, parent_id, update_parent)?;
    Ok(Json(updated_parent))
}

#[utoipa::path(delete, path = "/{id}", params(("id" = i32, Path, description = "ID Родителя которого требуется удалить")))]
async fn delete_parent(
    State(postgres_pool): State<PostgresPool>,
    Path(parent_id): Path<i32>,
) -> Result<Json<String>, AppError> {
    info!("Deleting parent");
    let delete_count = ParentService::delete(&postgres_pool, parent_id)?;
    if delete_count {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Parent not found".to_string()))
    }
}
