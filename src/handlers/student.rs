use axum::{Json, extract::State};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    db::PostgresPool,
    error::AppError,
    logic::services::student::StudentService,
    models::student::{NewStudent, Student},
};

pub fn router() -> OpenApiRouter<PostgresPool> {
    let dont_need_permissions = OpenApiRouter::new().routes(routes!(create_student));
    OpenApiRouter::new().merge(dont_need_permissions)
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/", request_body = NewStudent)]
async fn create_student(
    State(postgres_pool): State<PostgresPool>,
    Json(new_student): Json<NewStudent>,
) -> Result<Json<Student>, AppError> {
    info!("Creating new student");
    let new_student = StudentService::create(&postgres_pool, new_student)?;
    Ok(Json(new_student))
}
