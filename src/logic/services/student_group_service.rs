use tracing::{info, warn};

use crate::{
    db::{self, PostgresPool},
    error::AppError,
    logic::repositories::{
        lesson_repository::LessonRepository, student_group_repository::StudentGroupRepository,
    },
    models::{
        lesson::Lesson,
        student_group::{NewStudentGroup, StudentGroup, UpdateStudentGroup},
    },
};

pub struct StudentGroupService;

impl StudentGroupService {
    pub fn create(
        postgres_pool: &PostgresPool,
        new_student_group: NewStudentGroup,
    ) -> Result<StudentGroup, AppError> {
        let student_group = db::with_connection(postgres_pool, |connection| {
            StudentGroupRepository::create(connection, new_student_group)
        })?;
        info!(
            "Successfully created student group with ID {}",
            student_group.id
        );
        Ok(student_group)
    }

    pub fn get(
        postgres_pool: &PostgresPool,
        student_group_id: i32,
    ) -> Result<StudentGroup, AppError> {
        let student_group = db::with_connection(postgres_pool, |connection| {
            StudentGroupRepository::get(connection, student_group_id)
        })?;
        info!(
            "Student group with ID {} successfully get",
            student_group_id
        );
        Ok(student_group)
    }

    pub fn get_lessons(
        postgres_pool: &PostgresPool,
        student_group_id: i32,
    ) -> Result<Vec<Lesson>, AppError> {
        let lessons = db::with_connection(postgres_pool, |connection| {
            LessonRepository::get_lessons_by_group_id(connection, student_group_id)
        })?;
        info!("Got lessons for group with ID {}", student_group_id);
        Ok(lessons)
    }

    pub fn update(
        postgres_pool: &PostgresPool,
        student_group_id: i32,
        update_student_group: UpdateStudentGroup,
    ) -> Result<StudentGroup, AppError> {
        let updated_student_group = db::with_connection(postgres_pool, |connection| {
            StudentGroupRepository::update(connection, student_group_id, update_student_group)
        })?;
        info!(
            "Student group with ID {} was successfully updated",
            student_group_id
        );
        Ok(updated_student_group)
    }

    pub fn delete(postgres_pool: &PostgresPool, student_group_id: i32) -> Result<bool, AppError> {
        let deleted_count = db::with_connection(postgres_pool, |connection| {
            StudentGroupRepository::delete(connection, student_group_id)
        })?;

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
