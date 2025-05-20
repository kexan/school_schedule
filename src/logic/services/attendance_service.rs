use tracing::{info, warn};

use crate::{
    db::{self, PostgresPool},
    error::AppError,
    logic::repositories::attendance_repository::AttendanceRepository,
    models::attendance::{Attendance, NewAttendance, UpdateAttendance},
};

pub struct AttendanceService;

impl AttendanceService {
    pub fn create(
        postgres_pool: &PostgresPool,
        new_attendance: NewAttendance,
    ) -> Result<Attendance, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let attendance = AttendanceRepository::create(&mut connection, new_attendance)?;
        info!("Successfully created attendance with ID {}", attendance.id);
        Ok(attendance)
    }

    pub fn get(postgres_pool: &PostgresPool, attendance_id: i32) -> Result<Attendance, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let attendance = AttendanceRepository::get(&mut connection, attendance_id)?;
        info!("Attendance with ID {} successfully get", attendance_id);
        Ok(attendance)
    }

    pub fn update(
        postgres_pool: &PostgresPool,
        attendance_id: i32,
        update_attendance: UpdateAttendance,
    ) -> Result<Attendance, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let updated_attendance =
            AttendanceRepository::update(&mut connection, attendance_id, update_attendance)?;
        info!(
            "Attendance with ID {} was successfully updated",
            attendance_id
        );
        Ok(updated_attendance)
    }

    pub fn delete(postgres_pool: &PostgresPool, attendance_id: i32) -> Result<bool, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let deleted_count = AttendanceRepository::delete(&mut connection, attendance_id)?;

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
