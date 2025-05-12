use crate::models::teacher::{NewTeacher, Teacher};
use crate::schema::teachers::dsl::teachers;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};

pub struct TeacherRepository;

impl TeacherRepository {
    pub fn create(connection: &mut PgConnection, new_teacher: NewTeacher) -> QueryResult<Teacher> {
        diesel::insert_into(teachers)
            .values(new_teacher)
            .get_result(connection)
    }

    pub fn get(connection: &mut PgConnection, teacher_id: i32) -> QueryResult<Teacher> {
        teachers.find(teacher_id).first(connection)
    }

    pub fn update(
        connection: &mut PgConnection,
        teacher_id: i32,
        new_teacher: NewTeacher,
    ) -> QueryResult<Teacher> {
        diesel::update(teachers.find(teacher_id))
            .set(&new_teacher)
            .get_result(connection)
    }

    pub fn delete(connection: &mut PgConnection, teacher_id: i32) -> QueryResult<usize> {
        diesel::delete(teachers.find(teacher_id)).execute(connection)
    }
}
