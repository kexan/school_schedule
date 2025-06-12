use diesel::{QueryDsl, RunQueryDsl};

use crate::{
    db::PostgresPool,
    error::AppError,
    models::user::{NewUser, UpdateUser, User},
    schema::users,
};

#[derive(Clone)]
pub struct UserRepository {
    pool: PostgresPool,
}

impl UserRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, new_user: NewUser) -> Result<User, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut connection)?)
    }

    pub fn get(&self, user_id: i32) -> Result<User, AppError> {
        let mut connection = self.pool.get()?;
        Ok(users::table
            .find(user_id)
            .get_result::<User>(&mut connection)?)
    }

    pub fn update(&self, user_id: i32, updated_user: UpdateUser) -> Result<User, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::update(users::table.find(user_id))
            .set(&updated_user)
            .get_result::<User>(&mut connection)?)
    }

    pub fn delete(&self, user_id: i32) -> Result<usize, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::delete(users::table.find(user_id)).execute(&mut connection)?)
    }
}
