use crate::db::PostgresConnection;
use crate::models::student::{NewStudent, Student, UpdateStudent};
use crate::schema::students::dsl::students;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub struct StudentRepository;

impl StudentRepository {
    pub fn create(
        connection: &mut PostgresConnection,
        new_student: NewStudent,
    ) -> QueryResult<Student> {
        diesel::insert_into(students)
            .values(new_student)
            .get_result(connection)
    }

    pub fn get(connection: &mut PostgresConnection, student_id: i32) -> QueryResult<Student> {
        students.find(student_id).first(connection)
    }

    pub fn update(
        connection: &mut PostgresConnection,
        student_id: i32,
        update_student: UpdateStudent,
    ) -> QueryResult<Student> {
        diesel::update(students.find(student_id))
            .set(&update_student)
            .get_result(connection)
    }

    pub fn delete(connection: &mut PostgresConnection, student_id: i32) -> QueryResult<usize> {
        diesel::delete(students.find(student_id)).execute(connection)
    }
}
