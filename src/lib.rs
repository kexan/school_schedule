use axum::extract::FromRef;
use logic::services::{
    attendance_service::AttendanceService, lesson_service::LessonService,
    parent_service::ParentService, student_group_service::StudentGroupService,
    student_service::StudentService, teacher_service::TeacherService,
};

pub mod db;
pub mod error;
pub mod handlers;
pub mod logic;
pub mod models;
pub mod open_api;
pub mod schema;

#[derive(Clone)]
pub struct AppState {
    services: AppServices,
}

#[derive(Clone, FromRef)]
pub struct AppServices {
    pub lesson: LessonService,
    pub parent: ParentService,
    pub student: StudentService,
    pub student_group: StudentGroupService,
    pub teacher: TeacherService,
    pub attendance: AttendanceService,
}
