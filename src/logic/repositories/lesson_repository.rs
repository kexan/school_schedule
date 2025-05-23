use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::db::PostgresConnection;
use crate::models::lesson::{Lesson, NewLesson, UpdateLesson};
use crate::schema::lessons::dsl::lessons;
use crate::schema::lessons::student_group_id;

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

    pub fn get_lessons_by_group_id(
        connection: &mut PostgresConnection,
        id: i32,
    ) -> QueryResult<Vec<Lesson>> {
        lessons.filter(student_group_id.eq(id)).load(connection)
    }

    pub fn update(
        connection: &mut PostgresConnection,
        lesson_id: i32,
        updated_lesson: UpdateLesson,
    ) -> QueryResult<Lesson> {
        diesel::update(lessons.find(lesson_id))
            .set(&updated_lesson)
            .get_result(connection)
    }

    pub fn delete(connection: &mut PostgresConnection, lesson_id: i32) -> QueryResult<usize> {
        diesel::delete(lessons.find(lesson_id)).execute(connection)
    }
}
