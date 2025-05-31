use axum::extract::FromRef;
use tracing::{info, warn};

use crate::{
    AppState,
    error::AppError,
    logic::{
        repositories::lesson_repository::LessonRepository,
        services::attendance_service::AttendanceService,
    },
    models::lesson::{Lesson, LessonWithRelations, NewLesson, UpdateLesson},
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
        let lesson_full = self.lesson_repository.create(new_lesson)?;
        if let Some(student_group_id) = lesson_full.lesson.student_group_id {
            self.attendance_service
                .create_attendances_for_group(lesson_full.lesson.id, student_group_id)?;
        }
        info!(
            "Successfully created lesson with ID {}",
            lesson_full.lesson.id
        );
        Ok(lesson_full)
    }

    pub fn get(&self, lesson_id: i32) -> Result<LessonWithRelations, AppError> {
        let lesson = self.lesson_repository.get(lesson_id)?;
        info!("Lesson with ID {} successfully get", lesson_id);
        Ok(lesson)
    }

    pub fn get_lessons_by_group_id(&self, student_group_id: i32) -> Result<Vec<Lesson>, AppError> {
        let lessons = self
            .lesson_repository
            .get_lessons_by_group_id(student_group_id)?;
        info!("Got lessons for group with ID {}", student_group_id);
        Ok(lessons)
    }

    pub fn update(
        &self,
        lesson_id: i32,
        update_lesson: UpdateLesson,
    ) -> Result<LessonWithRelations, AppError> {
        let lesson = self.lesson_repository.get(lesson_id)?.lesson;
        if lesson.student_group_id != update_lesson.student_group_id {
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

impl FromRef<AppState> for LessonService {
    fn from_ref(state: &AppState) -> Self {
        state.services.lesson_service.clone()
    }
}
