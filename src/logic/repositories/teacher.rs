use crate::db::PostgresConnection;
use crate::models::teacher::{NewTeacher, Teacher, UpdateTeacher};
use crate::schema::teachers::dsl::teachers;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub struct TeacherRepository;

impl TeacherRepository {
    pub fn create(
        connection: &mut PostgresConnection,
        new_teacher: NewTeacher,
    ) -> QueryResult<Teacher> {
        diesel::insert_into(teachers)
            .values(new_teacher)
            .get_result(connection)
    }

    pub fn get(connection: &mut PostgresConnection, teacher_id: i32) -> QueryResult<Teacher> {
        teachers.find(teacher_id).first(connection)
    }

    pub fn update(
        connection: &mut PostgresConnection,
        teacher_id: i32,
        update_teacher: UpdateTeacher,
    ) -> QueryResult<Teacher> {
        diesel::update(teachers.find(teacher_id))
            .set(&update_teacher)
            .get_result(connection)
    }

    pub fn delete(connection: &mut PostgresConnection, teacher_id: i32) -> QueryResult<usize> {
        diesel::delete(teachers.find(teacher_id)).execute(connection)
    }
}
