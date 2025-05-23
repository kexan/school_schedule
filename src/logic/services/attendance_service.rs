use tracing::{info, warn};

use crate::{
    db,
    error::AppError,
    logic::repositories::attendance_repository::AttendanceRepository,
    models::attendance::{Attendance, NewAttendance, UpdateAttendance},
};

pub struct AttendanceService;

impl AttendanceService {
    pub fn create(
        postgres_pool: &db::PostgresPool,
        new_attendance: NewAttendance,
    ) -> Result<Attendance, AppError> {
        let attendance = db::with_connection(postgres_pool, |connection| {
            AttendanceRepository::create(connection, new_attendance)
        })?;
        info!("Successfully created attendance with ID {}", attendance.id);
        Ok(attendance)
    }

    pub fn get(
        postgres_pool: &db::PostgresPool,
        attendance_id: i32,
    ) -> Result<Attendance, AppError> {
        let attendance = db::with_connection(postgres_pool, |connection| {
            AttendanceRepository::get(connection, attendance_id)
        })?;
        info!("Attendance with ID {} successfully get", attendance_id);
        Ok(attendance)
    }

    pub fn update(
        postgres_pool: &db::PostgresPool,
        attendance_id: i32,
        update_attendance: UpdateAttendance,
    ) -> Result<Attendance, AppError> {
        let updated_attendance = db::with_connection(postgres_pool, |connection| {
            AttendanceRepository::update(connection, attendance_id, update_attendance)
        })?;
        info!(
            "Attendance with ID {} was successfully updated",
            attendance_id
        );
        Ok(updated_attendance)
    }

    pub fn delete(postgres_pool: &db::PostgresPool, attendance_id: i32) -> Result<bool, AppError> {
        let deleted_count = db::with_connection(postgres_pool, |connection| {
            AttendanceRepository::delete(connection, attendance_id)
        })?;

        if deleted_count > 0 {
            info!(
                "Attendance with ID {} was successfully deleted",
                attendance_id
            );
            Ok(true)
        } else {
            warn!("Attendance with ID {} not found", attendance_id);
            Ok(false)
        }
    }
}
