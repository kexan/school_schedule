use axum::extract::FromRef;
use tracing::{info, warn};

use crate::{
    AppState,
    error::AppError,
    logic::repositories::student_group_repository::StudentGroupRepository,
    models::student_group::{NewStudentGroup, StudentGroupWithRelations, UpdateStudentGroup},
};

#[derive(Clone)]
pub struct StudentGroupService {
    student_group_repository: StudentGroupRepository,
}

impl StudentGroupService {
    pub fn new(student_group_repository: StudentGroupRepository) -> Self {
        Self {
            student_group_repository,
        }
    }

    pub fn create(
        &self,
        new_student_group: NewStudentGroup,
    ) -> Result<StudentGroupWithRelations, AppError> {
        let student_group_full = self.student_group_repository.create(new_student_group)?;
        info!(
            "Successfully created student group with ID {}",
            student_group_full.student_group.id
        );
        Ok(student_group_full)
    }

    pub fn get(&self, student_group_id: i32) -> Result<StudentGroupWithRelations, AppError> {
        let student_group = self.student_group_repository.get(student_group_id)?;
        info!(
            "Student group with ID {} successfully get",
            student_group_id
        );
        Ok(student_group)
    }

    pub fn update(
        &self,
        student_group_id: i32,
        update_student_group: UpdateStudentGroup,
    ) -> Result<StudentGroupWithRelations, AppError> {
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
