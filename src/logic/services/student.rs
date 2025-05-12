use tracing::{info, warn};

use crate::{
    db::{self, PostgresPool},
    error::AppError,
    logic::repositories::student::StudentRepository,
    models::student::{NewStudent, Student, UpdateStudent},
};

pub struct StudentService;

impl StudentService {
    pub fn create(
        postgres_pool: &PostgresPool,
        new_student: NewStudent,
    ) -> Result<Student, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let student = StudentRepository::create(&mut connection, new_student)?;
        info!("Successfully created student with ID {}", student.id);
        Ok(student)
    }

    pub fn get(postgres_pool: &PostgresPool, student_id: i32) -> Result<Student, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let student = StudentRepository::get(&mut connection, student_id)?;
        info!("Student with ID {} successfully get", student_id);
        Ok(student)
    }

    pub fn update(
        postgres_pool: &PostgresPool,
        student_id: i32,
        update_student: UpdateStudent,
    ) -> Result<Student, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let updated_student =
            StudentRepository::update(&mut connection, student_id, update_student)?;
        info!("Student with ID {} was successfully updated", student_id);
        Ok(updated_student)
    }

    pub fn delete(postgres_pool: &PostgresPool, student_id: i32) -> Result<bool, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let deleted_count = StudentRepository::delete(&mut connection, student_id)?;

        if deleted_count > 0 {
            info!("Student with ID {} was successfully deleted", student_id);
            Ok(true)
        } else {
            warn!("Student with ID {} not found", student_id);
            Ok(false)
        }
    }
}
