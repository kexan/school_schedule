use diesel::prelude::*;

use crate::{
    db::PostgresPool,
    error::AppError,
    models::student_group::{NewStudentGroup, StudentGroup, UpdateStudentGroup},
    schema::student_groups::{self},
};

#[derive(Clone)]
pub struct StudentGroupRepository {
    pool: PostgresPool,
}

impl StudentGroupRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, new_student_group: NewStudentGroup) -> Result<StudentGroup, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::insert_into(student_groups::table)
            .values(new_student_group)
            .get_result::<StudentGroup>(&mut connection)?)
    }

    pub fn get(&self, student_group_id: i32) -> Result<StudentGroup, AppError> {
        let mut connection = self.pool.get()?;
        Ok(student_groups::table
            .find(student_group_id)
            .first::<StudentGroup>(&mut connection)?)
    }

    pub fn update(
        &self,
        student_group_id: i32,
        updated_student_group: UpdateStudentGroup,
    ) -> Result<StudentGroup, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::update(student_groups::table.find(student_group_id))
            .set(&updated_student_group)
            .get_result::<StudentGroup>(&mut connection)?)
    }

    pub fn delete(&self, student_group_id: i32) -> Result<usize, AppError> {
        let mut connection = self.pool.get()?;
        Ok(
            diesel::delete(student_groups::table.find(student_group_id))
                .execute(&mut connection)?,
        )
    }
}
