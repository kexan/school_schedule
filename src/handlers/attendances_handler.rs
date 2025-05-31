use axum::{
    Json,
    extract::{Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    error::AppError,
    logic::services::attendance_service::AttendanceService,
    models::attendance::{AttendanceWithRelations, NewAttendance, UpdateAttendance},
};

pub fn router() -> OpenApiRouter<AppState> {
    let dont_need_permissions = OpenApiRouter::new()
        .routes(routes!(
            create_attendance,
            get_attendance,
            update_attendance,
            delete_attendance,
        ))
        .routes(routes!(get_attendances_by_lesson));
    OpenApiRouter::new().merge(dont_need_permissions)
}

#[utoipa::path(post, path = "/", request_body = NewAttendance)]
async fn create_attendance(
    State(attendance_service): State<AttendanceService>,
    Json(new_attendance): Json<NewAttendance>,
) -> Result<Json<AttendanceWithRelations>, AppError> {
    info!("Creating new attendance");
    let created_attendance = attendance_service.create(new_attendance)?;
    Ok(Json(created_attendance))
}

#[utoipa::path(get, path = "/{id}", params(("id" = i32, Path, description = "ID запрашиваемого посещения")))]
async fn get_attendance(
    State(attendance_service): State<AttendanceService>,
    Path(attendance_id): Path<i32>,
) -> Result<Json<AttendanceWithRelations>, AppError> {
    info!("Getting attendance");
    let attendance = attendance_service.get(attendance_id)?;
    Ok(Json(attendance))
}

#[utoipa::path(get, path = "/lesson/{lesson_id}", params(("lesson_id" = i32, Path, description = "ID урока для которого запрашиваются посещения")))]
async fn get_attendances_by_lesson(
    State(attendance_service): State<AttendanceService>,
    Path(lesson_id): Path<i32>,
) -> Result<Json<Vec<AttendanceWithRelations>>, AppError> {
    info!("Getting attendances by lesson");
    let attendances = attendance_service.get_by_lesson_id(lesson_id)?;
    Ok(Json(attendances))
}

#[utoipa::path(put, path = "/{id}", params(("id" = i32, Path, description = "ID посещения которое требуется обновить")), request_body = UpdateAttendance)]
async fn update_attendance(
    State(attendance_service): State<AttendanceService>,
    Path(attendance_id): Path<i32>,
    Json(update_attendance): Json<UpdateAttendance>,
) -> Result<Json<AttendanceWithRelations>, AppError> {
    info!("Updating attendance");
    let updated_attendance = attendance_service.update(attendance_id, update_attendance)?;
    Ok(Json(updated_attendance))
}

#[utoipa::path(delete, path = "/{id}", params(("id" = i32, Path, description = "ID посещения которое требуется удалить")))]
async fn delete_attendance(
    State(attendance_service): State<AttendanceService>,
    Path(attendance_id): Path<i32>,
) -> Result<Json<String>, AppError> {
    info!("Deleting attendance");
    let deleted = attendance_service.delete(attendance_id)?;
    if deleted {
        Ok(Json("Successfully deleted".to_string()))
    } else {
        Ok(Json("Attendance not found".to_string()))
    }
}
