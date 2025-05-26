use diesel::prelude::*;

use crate::{
    db::PostgresPool,
    error::AppError,
    models::attendance::{Attendance, NewAttendance, UpdateAttendance},
    schema::attendances,
};

#[derive(Clone)]
pub struct AttendanceRepository {
    pool: PostgresPool,
}

impl AttendanceRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, new_attendance: NewAttendance) -> Result<Attendance, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::insert_into(attendances::table)
            .values(new_attendance)
            .get_result::<Attendance>(&mut connection)?)
    }

    pub fn batch_create(
        &self,
        new_attendances: Vec<NewAttendance>,
    ) -> Result<Vec<Attendance>, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::insert_into(attendances::table)
            .values(new_attendances)
            .get_results::<Attendance>(&mut connection)?)
    }

    pub fn get(&self, attendance_id: i32) -> Result<Attendance, AppError> {
        let mut connection = self.pool.get()?;
        Ok(attendances::table
            .find(attendance_id)
            .first::<Attendance>(&mut connection)?)
    }

    pub fn get_by_lesson_id(&self, lesson_id: i32) -> Result<Vec<Attendance>, AppError> {
        let mut connection = self.pool.get()?;
        Ok(attendances::table
            .filter(attendances::lesson_id.eq(lesson_id))
            .load::<Attendance>(&mut connection)?)
    }

    pub fn update(
        &self,
        attendance_id: i32,
        updated_attendance: UpdateAttendance,
    ) -> Result<Attendance, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::update(attendances::table.find(attendance_id))
            .set(&updated_attendance)
            .get_result::<Attendance>(&mut connection)?)
    }

    pub fn delete(&self, attendance_id: i32) -> Result<usize, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::delete(attendances::table.find(attendance_id)).execute(&mut connection)?)
    }

    pub fn delete_by_lesson_id(&self, lesson_id: i32) -> Result<usize, AppError> {
        let mut connection = self.pool.get()?;
        Ok(
            diesel::delete(attendances::table.filter(attendances::lesson_id.eq(lesson_id)))
                .execute(&mut connection)?,
        )
    }
}
