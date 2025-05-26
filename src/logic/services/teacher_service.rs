use tracing::info;

use crate::{
    error::AppError,
    logic::repositories::{
        document_repository::DocumentRepository, teacher_repository::TeacherRepository,
    },
    models::{
        document::Document,
        teacher::{NewTeacher, Teacher, UpdateTeacher},
    },
};

#[derive(Clone)]
pub struct TeacherService {
    teacher_repository: TeacherRepository,
    document_repository: DocumentRepository,
}

impl TeacherService {
    pub fn new(
        teacher_repository: TeacherRepository,
        document_repository: DocumentRepository,
    ) -> Self {
        Self {
            teacher_repository,
            document_repository,
        }
    }

    pub fn create(&self, new_teacher: NewTeacher) -> Result<Teacher, AppError> {
        let teacher = self.teacher_repository.create(new_teacher)?;
        info!("Successfully created teacher with ID {}", teacher.id);
        Ok(teacher)
    }

    pub fn get(&self, teacher_id: i32) -> Result<Teacher, AppError> {
        let teacher = self.teacher_repository.get(teacher_id)?;
        info!("Teacher with ID {} successfully get", teacher_id);
        Ok(teacher)
    }

    pub fn get_documents(&self, teacher_id: i32) -> Result<Vec<Document>, AppError> {
        let documents = self.document_repository.get_by_teacher_id(teacher_id)?;
        info!("Got all documents for teacher with ID {}", teacher_id);
        Ok(documents)
    }

    pub fn update(
        &self,
        teacher_id: i32,
        update_teacher: UpdateTeacher,
    ) -> Result<Teacher, AppError> {
        let updated_teacher = self.teacher_repository.update(teacher_id, update_teacher)?;
        info!("Successfully updated teacher with ID {}", teacher_id);
        Ok(updated_teacher)
    }

    pub fn delete(&self, teacher_id: i32) -> Result<bool, AppError> {
        let deleted_count = self.teacher_repository.delete(teacher_id)?;

        if deleted_count > 0 {
            info!("Teacher with ID {} successfully deleted", teacher_id);
            Ok(true)
        } else {
            info!("Teacher with ID {} not found", teacher_id);
            Ok(false)
        }
    }
}
