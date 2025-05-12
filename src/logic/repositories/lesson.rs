use diesel::{QueryDsl, QueryResult, RunQueryDsl};

use crate::db::PostgresConnection;
use crate::models::lesson::{Lesson, NewLesson};
use crate::schema::lessons::dsl::lessons;

pub struct LessonRepository;

impl LessonRepository {
    pub fn create(
        connection: &mut PostgresConnection,
        new_lesson: NewLesson,
    ) -> QueryResult<Lesson> {
        diesel::insert_into(lessons)
            .values(new_lesson)
            .get_result(connection)
    }

    pub fn get(connection: &mut PostgresConnection, lesson_id: i32) -> QueryResult<Lesson> {
        lessons.find(lesson_id).first(connection)
    }

    pub fn update(
        connection: &mut PostgresConnection,
        lesson_id: i32,
        new_lesson: NewLesson,
    ) -> QueryResult<Lesson> {
        diesel::update(lessons.find(lesson_id))
            .set(&new_lesson)
            .get_result(connection)
    }

    pub fn delete(connection: &mut PostgresConnection, lesson_id: i32) -> QueryResult<usize> {
        diesel::delete(lessons.find(lesson_id)).execute(connection)
    }
}
