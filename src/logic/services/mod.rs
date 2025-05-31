use attendance_service::AttendanceService;
use document_service::DocumentService;
use lesson_service::LessonService;
use parent_service::ParentService;
use student_group_service::StudentGroupService;
use student_service::StudentService;
use teacher_service::TeacherService;

use super::repositories::{
    attendance_repository::AttendanceRepository, document_repository::DocumentRepository,
    lesson_repository::LessonRepository, parent_repository::ParentRepository,
    student_group_repository::StudentGroupRepository, student_repository::StudentRepository,
    teacher_repository::TeacherRepository,
};
use crate::{AppServices, db::PostgresPool};

pub mod attendance_service;
pub mod document_service;
pub mod lesson_service;
pub mod parent_service;
pub mod student_group_service;
pub mod student_service;
pub mod teacher_service;

pub fn init_app_services(pool: PostgresPool) -> AppServices {
    let lesson_repo = LessonRepository::new(pool.clone());
    let parent_repo = ParentRepository::new(pool.clone());
    let student_repo = StudentRepository::new(pool.clone());
    let student_group_repo = StudentGroupRepository::new(pool.clone());
    let teacher_repo = TeacherRepository::new(pool.clone());
    let attendance_repo = AttendanceRepository::new(pool.clone());
    let document_repo = DocumentRepository::new(pool.clone());

    let lesson_service = LessonService::new(
        lesson_repo.clone(),
        AttendanceService::new(
            attendance_repo.clone(),
            StudentService::new(student_repo.clone()),
        ),
    );
    let parent_service = ParentService::new(parent_repo);
    let student_service = StudentService::new(student_repo.clone());
    let student_group_service = StudentGroupService::new(student_group_repo);
    let teacher_service = TeacherService::new(teacher_repo);
    let attendance_service = AttendanceService::new(attendance_repo, student_service.clone());
    let document_service = DocumentService::new(document_repo);

    AppServices {
        lesson_service,
        parent_service,
        student_service,
        student_group_service,
        teacher_service,
        attendance_service,
        document_service,
    }
}
