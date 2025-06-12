use logic::services::{
    attendance_service::AttendanceService, document_service::DocumentService,
    lesson_service::LessonService, parent_service::ParentService,
    student_group_service::StudentGroupService, student_service::StudentService,
    teacher_service::TeacherService,
};

use crate::logic::services::user_service::UserService;

pub mod auth;
pub mod db;
pub mod error;
pub mod handlers;
pub mod logic;
pub mod models;
pub mod open_api;
pub mod schema;

#[derive(Clone)]
pub struct AppState {
    pub services: AppServices,
}

#[derive(Clone)]
pub struct AppServices {
    pub lesson_service: LessonService,
    pub parent_service: ParentService,
    pub student_service: StudentService,
    pub student_group_service: StudentGroupService,
    pub teacher_service: TeacherService,
    pub attendance_service: AttendanceService,
    pub document_service: DocumentService,
    pub user_service: UserService,
}
