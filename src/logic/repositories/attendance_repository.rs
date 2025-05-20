use diesel::{QueryDsl, QueryResult, RunQueryDsl};

use crate::{
    db::PostgresConnection,
    models::attendance::{Attendance, NewAttendance, UpdateAttendance},
    schema::attendances::dsl::attendances,
};

pub struct AttendanceRepository;

impl AttendanceRepository {
    pub fn create(
        connection: &mut PostgresConnection,
        new_attendance: NewAttendance,
    ) -> QueryResult<Attendance> {
        diesel::insert_into(attendances)
            .values(new_attendance)
            .get_result(connection)
    }

    pub fn get(connection: &mut PostgresConnection, attendance_id: i32) -> QueryResult<Attendance> {
        attendances.find(attendance_id).first(connection)
    }

    pub fn update(
        connection: &mut PostgresConnection,
        attendance_id: i32,
        updated_attendance: UpdateAttendance,
    ) -> QueryResult<Attendance> {
        diesel::update(attendances.find(attendance_id))
            .set(&updated_attendance)
            .get_result(connection)
    }

    pub fn delete(connection: &mut PostgresConnection, attendance_id: i32) -> QueryResult<usize> {
        diesel::delete(attendances.find(attendance_id)).execute(connection)
    }
}
