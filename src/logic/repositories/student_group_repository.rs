use diesel::prelude::*;

use crate::{
    db::PostgresPool,
    error::AppError,
    models::{
        student_group::{
            NewStudentGroup, StudentGroup, StudentGroupWithRelations, UpdateStudentGroup,
        },
        teacher::Teacher,
    },
    schema::{
        student_groups::{self},
        teachers,
    },
};

use super::single_result;

#[derive(Clone)]
pub struct StudentGroupRepository {
    pool: PostgresPool,
}

impl StudentGroupRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub fn create(
        &self,
        new_student_group: NewStudentGroup,
    ) -> Result<StudentGroupWithRelations, AppError> {
        let mut connection = self.pool.get()?;
        let student_group_id = diesel::insert_into(student_groups::table)
            .values(&new_student_group)
            .returning(student_groups::id)
            .get_result::<i32>(&mut connection)?;

        self.get(student_group_id)
    }

    pub fn get(&self, student_group_id: i32) -> Result<StudentGroupWithRelations, AppError> {
        let query = student_groups::table
            .filter(student_groups::id.eq(student_group_id))
            .into_boxed();

        single_result(self.load_with_relations(query)?)
    }

    pub fn update(
        &self,
        student_group_id: i32,
        updated_student_group: UpdateStudentGroup,
    ) -> Result<StudentGroupWithRelations, AppError> {
        let mut connection = self.pool.get()?;
        diesel::update(student_groups::table.find(student_group_id))
            .set(&updated_student_group)
            .execute(&mut connection)?;

        self.get(student_group_id)
    }

    pub fn delete(&self, student_group_id: i32) -> Result<usize, AppError> {
        let mut connection = self.pool.get()?;
        Ok(
            diesel::delete(student_groups::table.find(student_group_id))
                .execute(&mut connection)?,
        )
    }

    fn load_with_relations(
        &self,
        query: student_groups::BoxedQuery<'_, diesel::pg::Pg>,
    ) -> Result<Vec<StudentGroupWithRelations>, AppError> {
        let mut connection = self.pool.get()?;
        let results = query
            .left_join(teachers::table)
            .select((StudentGroup::as_select(), Option::<Teacher>::as_select()))
            .load::<(StudentGroup, Option<Teacher>)>(&mut connection)?
            .into_iter()
            .map(|(student_group, teacher)| StudentGroupWithRelations {
                student_group,
                teacher,
            })
            .collect();

        Ok(results)
    }
}
