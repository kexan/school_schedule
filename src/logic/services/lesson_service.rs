use tracing::{info, warn};

use crate::{
    db::{self, PostgresPool},
    error::AppError,
    logic::repositories::{
        attendance_repository::AttendanceRepository, lesson_repository::LessonRepository,
    },
    models::{
        attendance::Attendance,
        lesson::{Lesson, NewLesson, UpdateLesson},
    },
};

pub struct LessonService;

impl LessonService {
    pub fn create(postgres_pool: &PostgresPool, new_lesson: NewLesson) -> Result<Lesson, AppError> {
        let lesson = db::with_connection(postgres_pool, |connection| {
            LessonRepository::create(connection, new_lesson)
        })?;
        info!("Successfully created lesson with ID {}", lesson.id);
        Ok(lesson)
    }

    pub fn get(postgres_pool: &PostgresPool, lesson_id: i32) -> Result<Lesson, AppError> {
        let lesson = db::with_connection(postgres_pool, |connection| {
            LessonRepository::get(connection, lesson_id)
        })?;
        info!("Lesson with ID {} successfully get", lesson_id);
        Ok(lesson)
    }

    pub fn get_attendances(
        postgres_pool: &PostgresPool,
        lesson_id: i32,
    ) -> Result<Vec<Attendance>, AppError> {
        let attendances = db::with_connection(postgres_pool, |connection| {
            AttendanceRepository::get_by_lesson_id(connection, lesson_id)
        })?;
        info!("Got attendances for lesson with ID {}", lesson_id);
        Ok(attendances)
    }

    pub fn update(
        postgres_pool: &PostgresPool,
        lesson_id: i32,
        update_lesson: UpdateLesson,
    ) -> Result<Lesson, AppError> {
        let updated_lesson = db::with_connection(postgres_pool, |connection| {
            LessonRepository::update(connection, lesson_id, update_lesson)
        })?;
        info!("Lesson with ID {} was successfully updated", lesson_id);
        Ok(updated_lesson)
    }

    pub fn delete(postgres_pool: &PostgresPool, lesson_id: i32) -> Result<bool, AppError> {
        let deleted_count = db::with_connection(postgres_pool, |connection| {
            LessonRepository::delete(connection, lesson_id)
        })?;

        if deleted_count > 0 {
            info!("Lesson with ID {} was successfully deleted", lesson_id);
            Ok(true)
        } else {
            warn!("Lesson with ID {} not found", lesson_id);
            Ok(false)
        }
    }
}
