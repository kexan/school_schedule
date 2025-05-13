use tracing::{info, warn};

use crate::{
    db::{self, PostgresPool},
    error::AppError,
    logic::repositories::student_group_repository::StudentGroupRepository,
    models::student_group::{NewStudentGroup, StudentGroup, UpdateStudentGroup},
};

pub struct StudentGroupService;

impl StudentGroupService {
    pub fn create(
        postgres_pool: &PostgresPool,
        new_student_group: NewStudentGroup,
    ) -> Result<StudentGroup, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let student_group = StudentGroupRepository::create(&mut connection, new_student_group)?;
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
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let student_group = StudentGroupRepository::get(&mut connection, student_group_id)?;
        info!(
            "Student group with ID {} successfully get",
            student_group_id
        );
        Ok(student_group)
    }

    pub fn update(
        postgres_pool: &PostgresPool,
        student_group_id: i32,
        update_student_group: UpdateStudentGroup,
    ) -> Result<StudentGroup, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let updated_student_group = StudentGroupRepository::update(
            &mut connection,
            student_group_id,
            update_student_group,
        )?;
        info!(
            "Student group with ID {} was successfully updated",
            student_group_id
        );
        Ok(updated_student_group)
    }

    pub fn delete(postgres_pool: &PostgresPool, student_id: i32) -> Result<bool, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let deleted_count = StudentGroupRepository::delete(&mut connection, student_id)?;

        if deleted_count > 0 {
            info!(
                "Student group with ID {} was successfully deleted",
                student_id
            );
            Ok(true)
        } else {
            warn!("Student group with ID {} not found", student_id);
            Ok(false)
        }
    }
}
