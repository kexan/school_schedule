use tracing::{info, warn};

use crate::{
    db::{self, PostgresPool},
    error::AppError,
    logic::repositories::student_repository::StudentRepository,
    models::student::{NewStudent, Student, UpdateStudent},
};

pub struct StudentService;

impl StudentService {
    pub fn create(
        postgres_pool: &PostgresPool,
        new_student: NewStudent,
    ) -> Result<Student, AppError> {
        let student = db::with_connection(postgres_pool, |connection| {
            StudentRepository::create(connection, new_student)
        })?;
        info!("Successfully created student with ID {}", student.id);
        Ok(student)
    }

    pub fn get(postgres_pool: &PostgresPool, student_id: i32) -> Result<Student, AppError> {
        let student = db::with_connection(postgres_pool, |connection| {
            StudentRepository::get(connection, student_id)
        })?;
        info!("Student with ID {} successfully get", student_id);
        Ok(student)
    }

    pub fn get_students_from_group(
        postgres_pool: &PostgresPool,
        student_group_id: i32,
    ) -> Result<Vec<Student>, AppError> {
        let students = db::with_connection(postgres_pool, |connection| {
            StudentRepository::get_students_by_group_id(connection, student_group_id)
        })?;
        info!("Got students from group with ID {}", student_group_id);
        Ok(students)
    }

    pub fn update(
        postgres_pool: &PostgresPool,
        student_id: i32,
        update_student: UpdateStudent,
    ) -> Result<Student, AppError> {
        let updated_student = db::with_connection(postgres_pool, |connection| {
            StudentRepository::update(connection, student_id, update_student)
        })?;
        info!("Student with ID {} was successfully updated", student_id);
        Ok(updated_student)
    }

    pub fn delete(postgres_pool: &PostgresPool, student_id: i32) -> Result<bool, AppError> {
        let deleted_count = db::with_connection(postgres_pool, |connection| {
            StudentRepository::delete(connection, student_id)
        })?;

        if deleted_count > 0 {
            info!("Student with ID {} was successfully deleted", student_id);
            Ok(true)
        } else {
            warn!("Student with ID {} not found", student_id);
            Ok(false)
        }
    }
}
