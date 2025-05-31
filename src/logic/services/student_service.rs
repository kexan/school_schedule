use axum::extract::FromRef;
use tracing::{info, warn};

use crate::{
    AppState,
    error::AppError,
    logic::repositories::student_repository::StudentRepository,
    models::student::{NewStudent, StudentWithRelations, UpdateStudent},
};

#[derive(Clone)]
pub struct StudentService {
    student_repository: StudentRepository,
}

impl StudentService {
    pub fn new(student_repository: StudentRepository) -> Self {
        Self { student_repository }
    }

    pub fn create(&self, new_student: NewStudent) -> Result<StudentWithRelations, AppError> {
        let student_full = self.student_repository.create(new_student)?;
        info!(
            "Successfully created student with ID {}",
            student_full.student.id
        );
        Ok(student_full)
    }

    pub fn get(&self, student_id: i32) -> Result<StudentWithRelations, AppError> {
        let student = self.student_repository.get(student_id)?;
        info!("Student with ID {} successfully get", student_id);
        Ok(student)
    }

    pub fn get_students_from_group(
        &self,
        student_group_id: i32,
    ) -> Result<Vec<StudentWithRelations>, AppError> {
        let students = self
            .student_repository
            .get_students_by_group_id(student_group_id)?;
        info!("Got students from group with ID {}", student_group_id);
        Ok(students)
    }

    pub fn update(
        &self,
        student_id: i32,
        update_student: UpdateStudent,
    ) -> Result<StudentWithRelations, AppError> {
        let updated_student = self.student_repository.update(student_id, update_student)?;
        info!("Student with ID {} was successfully updated", student_id);
        Ok(updated_student)
    }

    pub fn delete(&self, student_id: i32) -> Result<bool, AppError> {
        let deleted_count = self.student_repository.delete(student_id)?;

        if deleted_count > 0 {
            info!("Student with ID {} was successfully deleted", student_id);
            Ok(true)
        } else {
            warn!("Student with ID {} not found", student_id);
            Ok(false)
        }
    }
}

impl FromRef<AppState> for StudentService {
    fn from_ref(state: &AppState) -> Self {
        state.services.student_service.clone()
    }
}
