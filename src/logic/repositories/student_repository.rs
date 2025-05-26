use diesel::prelude::*;

use crate::{
    db::PostgresPool,
    error::AppError,
    models::student::{NewStudent, Student, UpdateStudent},
    schema::students::{self},
};

#[derive(Clone)]
pub struct StudentRepository {
    pool: PostgresPool,
}

impl StudentRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, new_student: NewStudent) -> Result<Student, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::insert_into(students::table)
            .values(new_student)
            .get_result::<Student>(&mut connection)?)
    }

    pub fn get(&self, student_id: i32) -> Result<Student, AppError> {
        let mut connection = self.pool.get()?;
        Ok(students::table
            .find(student_id)
            .first::<Student>(&mut connection)?)
    }

    pub fn get_students_by_group_id(&self, group_id: i32) -> Result<Vec<Student>, AppError> {
        let mut connection = self.pool.get()?;
        Ok(students::table
            .filter(students::student_group_id.eq(group_id))
            .load::<Student>(&mut connection)?)
    }

    pub fn update(
        &self,
        student_id: i32,
        updated_student: UpdateStudent,
    ) -> Result<Student, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::update(students::table.find(student_id))
            .set(&updated_student)
            .get_result::<Student>(&mut connection)?)
    }

    pub fn delete(&self, student_id: i32) -> Result<usize, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::delete(students::table.find(student_id)).execute(&mut connection)?)
    }
}
