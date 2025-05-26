use tracing::{info, warn};

use crate::{
    db::{self, PostgresPool},
    error::AppError,
    logic::repositories::attendance_repository::AttendanceRepository,
    models::attendance::{Attendance, NewAttendance, UpdateAttendance},
};

use super::student_service::StudentService;

#[derive(Clone)]
pub struct AttendanceService;

impl AttendanceService {
    pub fn create(
        postgres_pool: &PostgresPool,
        new_attendance: NewAttendance,
    ) -> Result<Attendance, AppError> {
        let attendance = db::with_connection(postgres_pool, |connection| {
            AttendanceRepository::create(connection, new_attendance)
        })?;
        info!("Successfully created attendance with ID {}", attendance.id);
        Ok(attendance)
    }

    pub fn create_attendances_for_group(
        postgres_pool: &PostgresPool,
        lesson_id: i32,
        student_group_id: i32,
    ) -> Result<Vec<Attendance>, AppError> {
        let students = StudentService::get_students_from_group(postgres_pool, student_group_id)?;
        let mut new_attendances = Vec::new();

        for student in students {
            let new_attendance = NewAttendance {
                lesson_id,
                student_id: student.id,
                is_present: false,
                skip_reason: None,
            };

            new_attendances.push(new_attendance);
        }

        let attendances = db::with_connection(postgres_pool, |connection| {
            AttendanceRepository::batch_create(connection, new_attendances)
        })?;
        info!(
            "Successfully created all attendances for lesson {}, for students in group {}",
            lesson_id, student_group_id
        );
        Ok(attendances)
    }

    pub fn get(postgres_pool: &PostgresPool, attendance_id: i32) -> Result<Attendance, AppError> {
        let attendance = db::with_connection(postgres_pool, |connection| {
            AttendanceRepository::get(connection, attendance_id)
        })?;
        info!("Attendance with ID {} successfully get", attendance_id);
        Ok(attendance)
    }

    pub fn get_by_lesson_id(
        postgres_pool: &PostgresPool,
        lesson_id: i32,
    ) -> Result<Vec<Attendance>, AppError> {
        let attendances = db::with_connection(postgres_pool, |connection| {
            AttendanceRepository::get_by_lesson_id(connection, lesson_id)
        })?;
        info!("Got attendances for lesson with ID {}", lesson_id);
        Ok(attendances)
    }

    pub fn update(
        postgres_pool: &PostgresPool,
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

    pub fn delete(postgres_pool: &PostgresPool, attendance_id: i32) -> Result<bool, AppError> {
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

    pub fn delete_by_lesson_id(
        postgres_pool: &PostgresPool,
        lesson_id: i32,
    ) -> Result<bool, AppError> {
        let deleted_count = db::with_connection(postgres_pool, |connection| {
            AttendanceRepository::delete_by_lesson_id(connection, lesson_id)
        })?;

        if deleted_count > 0 {
            info!(
                "Deleted {} attendances for lesson {}",
                deleted_count, lesson_id
            );
            Ok(true)
        } else {
            warn!("Not found attendances for lesson {}", lesson_id);
            Ok(false)
        }
    }
}
