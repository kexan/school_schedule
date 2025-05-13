use diesel::{QueryDsl, QueryResult, RunQueryDsl};

use crate::models::student_group::UpdateStudentGroup;
use crate::schema::student_groups::dsl::student_groups;
use crate::{
    db::PostgresConnection,
    models::student_group::{NewStudentGroup, StudentGroup},
};

pub struct StudentGroupRepository;

impl StudentGroupRepository {
    pub fn create(
        connection: &mut PostgresConnection,
        new_student_group: NewStudentGroup,
    ) -> QueryResult<StudentGroup> {
        diesel::insert_into(student_groups)
            .values(new_student_group)
            .get_result(connection)
    }

    pub fn get(
        connection: &mut PostgresConnection,
        student_group_id: i32,
    ) -> QueryResult<StudentGroup> {
        student_groups.find(student_group_id).first(connection)
    }

    pub fn update(
        connection: &mut PostgresConnection,
        student_group_id: i32,
        updated_student_group: UpdateStudentGroup,
    ) -> QueryResult<StudentGroup> {
        diesel::update(student_groups.find(student_group_id))
            .set(&updated_student_group)
            .get_result(connection)
    }

    pub fn delete(
        connection: &mut PostgresConnection,
        student_group_id: i32,
    ) -> QueryResult<usize> {
        diesel::delete(student_groups.find(student_group_id)).execute(connection)
    }
}
