use tracing::{info, warn};

use crate::{
    error::AppError,
    logic::{
        repositories::lesson_repository::LessonRepository,
        services::attendance_service::AttendanceService,
    },
    models::{
        attendance::Attendance,
        lesson::{LessonWithRelations, NewLesson, UpdateLesson},
    },
};

#[derive(Clone)]
pub struct LessonService {
    lesson_repository: LessonRepository,
    attendance_service: AttendanceService,
}

impl LessonService {
    pub fn new(lesson_repository: LessonRepository, attendance_service: AttendanceService) -> Self {
        Self {
            lesson_repository,
            attendance_service,
        }
    }

    pub fn create(&self, new_lesson: NewLesson) -> Result<LessonWithRelations, AppError> {
        //TODO: вот это бы надо переделать чтобы оно все выполнялось в рамках одной транзакции
        let lesson_response = self.lesson_repository.create(new_lesson)?;
        if let Some(student_group_id) = lesson_response.lesson.student_group_id {
            self.attendance_service
                .create_attendances_for_group(lesson_response.lesson.id, student_group_id)?;
        }
        info!(
            "Successfully created lesson with ID {}",
            lesson_response.lesson.id
        );
        Ok(lesson_response)
    }

    pub fn get(&self, lesson_id: i32) -> Result<LessonWithRelations, AppError> {
        let lesson_response = self.lesson_repository.get(lesson_id)?;
        info!("Lesson with ID {} successfully get", lesson_id);
        Ok(lesson_response)
    }

    pub fn get_attendances(&self, lesson_id: i32) -> Result<Vec<Attendance>, AppError> {
        let attendances = self.attendance_service.get_by_lesson_id(lesson_id)?;
        info!("Got attendances for lesson with ID {}", lesson_id);
        Ok(attendances)
    }

    pub fn update(
        &self,
        lesson_id: i32,
        update_lesson: UpdateLesson,
    ) -> Result<LessonWithRelations, AppError> {
        //TODO: вот это бы надо переделать чтобы оно все выполнялось в рамках одной транзакции
        let lesson_response = self.lesson_repository.get(lesson_id)?;
        if lesson_response.lesson.student_group_id != update_lesson.student_group_id {
            self.attendance_service.delete_by_lesson_id(lesson_id)?;

            if let Some(student_group_id) = update_lesson.student_group_id {
                self.attendance_service
                    .create_attendances_for_group(lesson_id, student_group_id)?;
            }
        }

        let updated_lesson = self.lesson_repository.update(lesson_id, update_lesson)?;
        info!("Lesson with ID {} was successfully updated", lesson_id);
        Ok(updated_lesson)
    }

    pub fn delete(&self, lesson_id: i32) -> Result<bool, AppError> {
        let deleted_count = self.lesson_repository.delete(lesson_id)?;
        if deleted_count > 0 {
            info!("Lesson with ID {} was successfully deleted", lesson_id);
            Ok(true)
        } else {
            warn!("Lesson with ID {} not found", lesson_id);
            Ok(false)
        }
    }
}
