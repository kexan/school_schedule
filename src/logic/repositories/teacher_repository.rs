use diesel::prelude::*;

use crate::{
    db::PostgresPool,
    error::AppError,
    models::teacher::{NewTeacher, Teacher, UpdateTeacher},
    schema::teachers::{self},
};

#[derive(Clone)]
pub struct TeacherRepository {
    pool: PostgresPool,
}

impl TeacherRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, new_teacher: NewTeacher) -> Result<Teacher, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::insert_into(teachers::table)
            .values(new_teacher)
            .get_result::<Teacher>(&mut connection)?)
    }

    pub fn get(&self, teacher_id: i32) -> Result<Teacher, AppError> {
        let mut connection = self.pool.get()?;
        Ok(teachers::table
            .find(teacher_id)
            .first::<Teacher>(&mut connection)?)
    }

    pub fn update(
        &self,
        teacher_id: i32,
        updated_teacher: UpdateTeacher,
    ) -> Result<Teacher, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::update(teachers::table.find(teacher_id))
            .set(&updated_teacher)
            .get_result::<Teacher>(&mut connection)?)
    }

    pub fn delete(&self, teacher_id: i32) -> Result<usize, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::delete(teachers::table.find(teacher_id)).execute(&mut connection)?)
    }
}
