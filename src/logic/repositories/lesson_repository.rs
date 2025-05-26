use diesel::prelude::*;

use crate::db::PostgresPool;
use crate::error::AppError;
use crate::models::lesson::{Lesson, LessonWithRelations, NewLesson, UpdateLesson};
use crate::models::student_group::StudentGroup;
use crate::schema::lessons::{self};
use crate::schema::student_groups;

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

        let (lesson, student_group) = lessons::table
            .left_join(student_groups::table)
            .filter(lessons::id.eq(lesson_id))
            .select((Lesson::as_select(), Option::<StudentGroup>::as_select()))
            .first::<(Lesson, Option<StudentGroup>)>(&mut connection)?;

        Ok(LessonWithRelations {
            lesson,
            student_group,
        })
    }

    pub fn get(&self, lesson_id: i32) -> Result<LessonWithRelations, AppError> {
        let mut connection = self.pool.get()?;
        let (lesson, student_group) = lessons::table
            .left_join(student_groups::table)
            .filter(lessons::id.eq(lesson_id))
            .select((Lesson::as_select(), Option::<StudentGroup>::as_select()))
            .first::<(Lesson, Option<StudentGroup>)>(&mut connection)?;
        Ok(LessonWithRelations {
            lesson,
            student_group,
        })
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

        let (lesson, student_group) = lessons::table
            .left_join(student_groups::table)
            .filter(lessons::id.eq(lesson_id))
            .select((Lesson::as_select(), Option::<StudentGroup>::as_select()))
            .first::<(Lesson, Option<StudentGroup>)>(&mut connection)?;

        Ok(LessonWithRelations {
            lesson,
            student_group,
        })
    }

    pub fn delete(&self, lesson_id: i32) -> Result<usize, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::delete(lessons::table.find(lesson_id)).execute(&mut connection)?)
    }
}
