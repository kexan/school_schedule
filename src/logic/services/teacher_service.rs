use tracing::info;

use crate::{
    db::{self, PostgresPool},
    error::AppError,
    logic::repositories::{
        document_repository::DocumentRepository, teacher_repository::TeacherRepository,
    },
    models::{
        document::Document,
        teacher::{NewTeacher, Teacher, UpdateTeacher},
    },
};

pub struct TeacherService;

impl TeacherService {
    pub fn create(
        postgres_pool: &PostgresPool,
        new_teacher: NewTeacher,
    ) -> Result<Teacher, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let teacher = TeacherRepository::create(&mut connection, new_teacher)?;
        info!("Successfully created teacher with ID {}", teacher.id);
        Ok(teacher)
    }

    pub fn get(postgres_pool: &PostgresPool, teacher_id: i32) -> Result<Teacher, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let teacher = TeacherRepository::get(&mut connection, teacher_id)?;
        info!("Teacher with ID {} successfully get", teacher.id);
        Ok(teacher)
    }

    pub fn get_documents(
        postgres_pool: &PostgresPool,
        teacher_id: i32,
    ) -> Result<Vec<Document>, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let documents = DocumentRepository::get_by_teacher_id(&mut connection, teacher_id)?;
        info!("Got all documents for teacher with ID {}", teacher_id);
        Ok(documents)
    }

    pub fn update(
        postgres_pool: &PostgresPool,
        teacher_id: i32,
        update_teacher: UpdateTeacher,
    ) -> Result<Teacher, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let updated_teacher =
            TeacherRepository::update(&mut connection, teacher_id, update_teacher)?;
        info!("Successfully updated teacher with ID {}", teacher_id);
        Ok(updated_teacher)
    }

    pub fn delete(postgres_pool: &PostgresPool, teacher_id: i32) -> Result<bool, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let deleted_count = TeacherRepository::delete(&mut connection, teacher_id)?;

        if deleted_count > 0 {
            info!("Teacher with ID {} successfully deleted", teacher_id);
            Ok(true)
        } else {
            info!("Teacher with ID {} not found", teacher_id);
            Ok(false)
        }
    }
}
