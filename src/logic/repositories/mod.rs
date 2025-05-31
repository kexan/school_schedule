use crate::error::AppError;

pub(super) mod attendance_repository;
pub(super) mod document_repository;
pub(super) mod lesson_repository;
pub(super) mod parent_repository;
pub(super) mod student_group_repository;
pub(super) mod student_repository;
pub(super) mod teacher_repository;

pub(super) fn single_result<T>(vec: Vec<T>) -> Result<T, AppError> {
    vec.into_iter()
        .next()
        .ok_or_else(|| AppError::NotFound("Record not found".to_string()))
}
