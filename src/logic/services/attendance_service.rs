use tracing::{info, warn};

use crate::{
    error::AppError,
    logic::{
        repositories::attendance_repository::AttendanceRepository,
        services::student_service::StudentService,
    },
    models::attendance::{Attendance, NewAttendance, UpdateAttendance},
};

#[derive(Clone)]
pub struct AttendanceService {
    attendance_repository: AttendanceRepository,
    student_service: StudentService,
}

impl AttendanceService {
    pub fn new(
        attendance_repository: AttendanceRepository,
        student_service: StudentService,
    ) -> Self {
        Self {
            attendance_repository,
            student_service,
        }
    }

    pub fn create(&self, new_attendance: NewAttendance) -> Result<Attendance, AppError> {
        let attendance = self.attendance_repository.create(new_attendance)?;
        info!("Successfully created attendance with ID {}", attendance.id);
        Ok(attendance)
    }

    pub fn create_attendances_for_group(
        &self,
        lesson_id: i32,
        student_group_id: i32,
    ) -> Result<Vec<Attendance>, AppError> {
        let students = self
            .student_service
            .get_students_from_group(student_group_id)?;
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

        let attendances = self.attendance_repository.batch_create(new_attendances)?;
        info!(
            "Successfully created all attendances for lesson {}, for students in group {}",
            lesson_id, student_group_id
        );
        Ok(attendances)
    }

    pub fn get(&self, attendance_id: i32) -> Result<Attendance, AppError> {
        let attendance = self.attendance_repository.get(attendance_id)?;
        info!("Attendance with ID {} successfully get", attendance_id);
        Ok(attendance)
    }

    pub fn get_by_lesson_id(&self, lesson_id: i32) -> Result<Vec<Attendance>, AppError> {
        let attendances = self.attendance_repository.get_by_lesson_id(lesson_id)?;
        info!("Got attendances for lesson with ID {}", lesson_id);
        Ok(attendances)
    }

    pub fn update(
        &self,
        attendance_id: i32,
        update_attendance: UpdateAttendance,
    ) -> Result<Attendance, AppError> {
        let updated_attendance = self
            .attendance_repository
            .update(attendance_id, update_attendance)?;
        info!(
            "Attendance with ID {} was successfully updated",
            attendance_id
        );
        Ok(updated_attendance)
    }

    pub fn delete(&self, attendance_id: i32) -> Result<bool, AppError> {
        let deleted_count = self.attendance_repository.delete(attendance_id)?;

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

    pub fn delete_by_lesson_id(&self, lesson_id: i32) -> Result<bool, AppError> {
        let deleted_count = self.attendance_repository.delete_by_lesson_id(lesson_id)?;

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
