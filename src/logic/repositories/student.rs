use crate::models::student::{NewStudent, Student};
use crate::schema::students::dsl::students;
use crate::schema::students::id;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};

pub struct StudentRepository;

impl StudentRepository {
    pub fn create(connection: &mut PgConnection, new_student: NewStudent) -> QueryResult<Student> {
        diesel::insert_into(students)
            .values(new_student)
            .get_result(connection)
    }

    pub fn get(connection: &mut PgConnection, student_id: i32) -> QueryResult<Student> {
        students.find(student_id).first(connection)
    }

    pub fn update(
        connection: &mut PgConnection,
        student_id: i32,
        new_student: NewStudent,
    ) -> QueryResult<Student> {
        diesel::update(students.filter(id.eq(student_id)))
            .set(&new_student)
            .get_result(connection)
    }

    pub fn delete(connection: &mut PgConnection, student_id: i32) -> QueryResult<usize> {
        diesel::delete(students.filter(id.eq(student_id))).execute(connection)
    }
}
