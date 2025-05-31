use diesel::{pg::Pg, prelude::*};

use crate::{
    db::PostgresPool,
    error::AppError,
    models::{
        lesson::{Lesson, LessonWithRelations, NewLesson, UpdateLesson},
        student_group::StudentGroup,
    },
    schema::lessons::{self},
    schema::student_groups,
};

use super::single_result;

#[derive(Clone)]
pub struct LessonRepository {
    pool: PostgresPool,
}

impl LessonRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, new_lesson: NewLesson) -> Result<LessonWithRelations, AppError> {
        let mut connection = self.pool.get()?;
        let lesson_id = diesel::insert_into(lessons::table)
            .values(&new_lesson)
            .returning(lessons::id)
            .get_result::<i32>(&mut connection)?;

        let query = lessons::table
            .filter(lessons::id.eq(lesson_id))
            .into_boxed();

        single_result(self.load_with_relations(query)?)
    }

    pub fn get(&self, lesson_id: i32) -> Result<LessonWithRelations, AppError> {
        let query = lessons::table
            .filter(lessons::id.eq(lesson_id))
            .into_boxed();

        single_result(self.load_with_relations(query)?)
    }

    pub fn get_lessons_by_group_id(&self, id: i32) -> Result<Vec<Lesson>, AppError> {
        let mut connection = self.pool.get()?;
        Ok(lessons::table
            .filter(lessons::student_group_id.eq(id))
            .select(Lesson::as_select())
            .load(&mut connection)?)
    }

    pub fn update(
        &self,
        lesson_id: i32,
        updated_lesson: UpdateLesson,
    ) -> Result<LessonWithRelations, AppError> {
        let mut connection = self.pool.get()?;
        diesel::update(lessons::table.find(lesson_id))
            .set(&updated_lesson)
            .execute(&mut connection)?;

        let query = lessons::table
            .filter(lessons::id.eq(lesson_id))
            .into_boxed();

        single_result(self.load_with_relations(query)?)
    }

    pub fn delete(&self, lesson_id: i32) -> Result<usize, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::delete(lessons::table.find(lesson_id)).execute(&mut connection)?)
    }

    fn load_with_relations(
        &self,
        query: lessons::BoxedQuery<'_, Pg>,
    ) -> Result<Vec<LessonWithRelations>, AppError> {
        let mut connection = self.pool.get()?;
        let results = query
            .left_join(student_groups::table)
            .select((Lesson::as_select(), Option::<StudentGroup>::as_select()))
            .load::<(Lesson, Option<StudentGroup>)>(&mut connection)?
            .into_iter()
            .map(|(lesson, student_group)| LessonWithRelations {
                lesson,
                student_group,
            })
            .collect();

        Ok(results)
    }
}
