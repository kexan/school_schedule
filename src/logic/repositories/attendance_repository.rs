use diesel::{pg::Pg, prelude::*};

use crate::{
    db::PostgresPool,
    error::AppError,
    models::{
        attendance::{Attendance, AttendanceWithRelations, NewAttendance, UpdateAttendance},
        lesson::Lesson,
        student::Student,
    },
    schema::{attendances, lessons, students},
};

use super::single_result;

#[derive(Clone)]
pub struct AttendanceRepository {
    pool: PostgresPool,
}

impl AttendanceRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub fn create(
        &self,
        new_attendance: NewAttendance,
    ) -> Result<AttendanceWithRelations, AppError> {
        let mut connection = self.pool.get()?;
        let new_attendance_id = diesel::insert_into(attendances::table)
            .values(&new_attendance)
            .returning(attendances::id)
            .get_result::<i32>(&mut connection)?;

        self.get(new_attendance_id)
    }

    pub fn batch_create(
        &self,
        new_attendances: Vec<NewAttendance>,
    ) -> Result<Vec<AttendanceWithRelations>, AppError> {
        let mut connection = self.pool.get()?;

        let inserted_ids: Vec<i32> = diesel::insert_into(attendances::table)
            .values(&new_attendances)
            .returning(attendances::id)
            .get_results(&mut connection)?;

        let query = attendances::table
            .filter(attendances::id.eq_any(inserted_ids))
            .into_boxed();

        self.load_with_relations(query)
    }

    pub fn get(&self, attendance_id: i32) -> Result<AttendanceWithRelations, AppError> {
        let query = attendances::table
            .filter(attendances::id.eq(attendance_id))
            .into_boxed();

        single_result(self.load_with_relations(query)?)
    }

    pub fn get_by_lesson_id(
        &self,
        lesson_id: i32,
    ) -> Result<Vec<AttendanceWithRelations>, AppError> {
        let query = attendances::table
            .filter(attendances::lesson_id.eq(lesson_id))
            .into_boxed();

        self.load_with_relations(query)
    }

    pub fn update(
        &self,
        attendance_id: i32,
        updated_attendance: UpdateAttendance,
    ) -> Result<AttendanceWithRelations, AppError> {
        let mut connection = self.pool.get()?;
        diesel::update(attendances::table.find(attendance_id))
            .set(&updated_attendance)
            .execute(&mut connection)?;

        self.get(attendance_id)
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

    fn load_with_relations(
        &self,
        query: attendances::BoxedQuery<'_, Pg>,
    ) -> Result<Vec<AttendanceWithRelations>, AppError> {
        let mut connection = self.pool.get()?;
        let results = query
            .inner_join(students::table)
            .inner_join(lessons::table)
            .select((
                Attendance::as_select(),
                Student::as_select(),
                Lesson::as_select(),
            ))
            .load::<(Attendance, Student, Lesson)>(&mut connection)?
            .into_iter()
            .map(|(a, s, l)| AttendanceWithRelations {
                attendance: a,
                student: s,
                lesson: l,
            })
            .collect();

        Ok(results)
    }
}
