use axum::extract::FromRef;
use tracing::{info, warn};

use crate::{
    AppState,
    error::AppError,
    logic::repositories::{
        lesson_repository::LessonRepository, student_group_repository::StudentGroupRepository,
    },
    models::{
        lesson::Lesson,
        student_group::{NewStudentGroup, StudentGroup, UpdateStudentGroup},
    },
};

#[derive(Clone)]
pub struct StudentGroupService {
    student_group_repository: StudentGroupRepository,
    lesson_repository: LessonRepository,
}

impl StudentGroupService {
    pub fn new(
        student_group_repository: StudentGroupRepository,
        lesson_repository: LessonRepository,
    ) -> Self {
        Self {
            student_group_repository,
            lesson_repository,
        }
    }

    pub fn create(&self, new_student_group: NewStudentGroup) -> Result<StudentGroup, AppError> {
        let student_group = self.student_group_repository.create(new_student_group)?;
        info!(
            "Successfully created student group with ID {}",
            student_group.id
        );
        Ok(student_group)
    }

    pub fn get(&self, student_group_id: i32) -> Result<StudentGroup, AppError> {
        let student_group = self.student_group_repository.get(student_group_id)?;
        info!(
            "Student group with ID {} successfully get",
            student_group_id
        );
        Ok(student_group)
    }

    pub fn get_lessons(&self, student_group_id: i32) -> Result<Vec<Lesson>, AppError> {
        let lessons = self
            .lesson_repository
            .get_lessons_by_group_id(student_group_id)?;
        info!("Got lessons for group with ID {}", student_group_id);
        Ok(lessons)
    }

    pub fn update(
        &self,
        student_group_id: i32,
        update_student_group: UpdateStudentGroup,
    ) -> Result<StudentGroup, AppError> {
        let updated_student_group = self
            .student_group_repository
            .update(student_group_id, update_student_group)?;
        info!(
            "Student group with ID {} was successfully updated",
            student_group_id
        );
        Ok(updated_student_group)
    }

    pub fn delete(&self, student_group_id: i32) -> Result<bool, AppError> {
        let deleted_count = self.student_group_repository.delete(student_group_id)?;

        if deleted_count > 0 {
            info!(
                "Student group with ID {} was successfully deleted",
                student_group_id
            );
            Ok(true)
        } else {
            warn!("Student group with ID {} not found", student_group_id);
            Ok(false)
        }
    }
}

impl FromRef<AppState> for StudentGroupService {
    fn from_ref(state: &AppState) -> Self {
        state.services.student_group_service.clone()
    }
}
