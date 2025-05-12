use tracing::{info, warn};

use crate::{
    db::{self, PostgresPool},
    error::AppError,
    logic::repositories::lesson::LessonRepository,
    models::lesson::{Lesson, NewLesson, UpdateLesson},
};

pub struct LessonService;

impl LessonService {
    pub fn create(postgres_pool: &PostgresPool, new_lesson: NewLesson) -> Result<Lesson, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let lesson = LessonRepository::create(&mut connection, new_lesson)?;
        info!("Successfully created lesson with ID {}", lesson.id);
        Ok(lesson)
    }

    pub fn get(postgres_pool: &PostgresPool, lesson_id: i32) -> Result<Lesson, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let lesson = LessonRepository::get(&mut connection, lesson_id)?;
        info!("Lesson with ID {} successfully get", lesson_id);
        Ok(lesson)
    }

    pub fn update(
        postgres_pool: &PostgresPool,
        lesson_id: i32,
        update_lesson: UpdateLesson,
    ) -> Result<Lesson, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let updated_lesson = LessonRepository::update(&mut connection, lesson_id, update_lesson)?;
        info!("Lesson with ID {} was successfully updated", lesson_id);
        Ok(updated_lesson)
    }

    pub fn delete(postgres_pool: &PostgresPool, lesson_id: i32) -> Result<bool, AppError> {
        let mut connection = db::get_postgres_connection(postgres_pool)?;
        let deleted_count = LessonRepository::delete(&mut connection, lesson_id)?;

        if deleted_count > 0 {
            info!("Lesson with ID {} was successfully deleted", lesson_id);
            Ok(true)
        } else {
            warn!("Lesson with ID {} not found", lesson_id);
            Ok(false)
        }
    }
}
