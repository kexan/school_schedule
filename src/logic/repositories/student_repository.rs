use crate::{
    db::PostgresPool,
    error::AppError,
    models::{
        parent::Parent,
        student::{NewStudent, Student, StudentWithRelations, UpdateStudent},
        student_group::StudentGroup,
    },
    schema::{
        parents, student_groups,
        students::{self},
    },
};
use diesel::{pg::Pg, prelude::*};

use super::single_result;

#[derive(Clone)]
pub struct StudentRepository {
    pool: PostgresPool,
}

impl StudentRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, new_student: NewStudent) -> Result<StudentWithRelations, AppError> {
        let mut connection = self.pool.get()?;
        let student_id = diesel::insert_into(students::table)
            .values(&new_student)
            .returning(students::id)
            .get_result::<i32>(&mut connection)?;

        self.get(student_id)
    }

    pub fn get(&self, student_id: i32) -> Result<StudentWithRelations, AppError> {
        let query = students::table
            .filter(students::id.eq(student_id))
            .into_boxed();

        single_result(self.load_with_relations(query)?)
    }

    pub fn get_students_by_group_id(
        &self,
        group_id: i32,
    ) -> Result<Vec<StudentWithRelations>, AppError> {
        let query = students::table
            .filter(students::student_group_id.eq(group_id))
            .into_boxed();

        self.load_with_relations(query)
    }

    pub fn update(
        &self,
        student_id: i32,
        updated_student: UpdateStudent,
    ) -> Result<StudentWithRelations, AppError> {
        let mut connection = self.pool.get()?;
        diesel::update(students::table.find(student_id))
            .set(&updated_student)
            .execute(&mut connection)?;

        self.get(student_id)
    }

    pub fn delete(&self, student_id: i32) -> Result<usize, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::delete(students::table.find(student_id)).execute(&mut connection)?)
    }

    fn load_with_relations(
        &self,
        query: students::BoxedQuery<'_, Pg>,
    ) -> Result<Vec<StudentWithRelations>, AppError> {
        let mut connection = self.pool.get()?;
        let results = query
            .left_join(parents::table)
            .left_join(student_groups::table)
            .select((
                Student::as_select(),
                Option::<Parent>::as_select(),
                Option::<StudentGroup>::as_select(),
            ))
            .load::<(Student, Option<Parent>, Option<StudentGroup>)>(&mut connection)?
            .into_iter()
            .map(|(student, parent, student_group)| StudentWithRelations {
                student,
                parent,
                student_group,
            })
            .collect();

        Ok(results)
    }
}
