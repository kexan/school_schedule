use diesel::prelude::*;

use crate::{
    db::PostgresPool,
    error::AppError,
    models::parent::{NewParent, Parent, UpdateParent},
    schema::parents::{self},
};

#[derive(Clone)]
pub struct ParentRepository {
    pool: PostgresPool,
}

impl ParentRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, new_parent: NewParent) -> Result<Parent, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::insert_into(parents::table)
            .values(new_parent)
            .get_result::<Parent>(&mut connection)?)
    }

    pub fn get(&self, parent_id: i32) -> Result<Parent, AppError> {
        let mut connection = self.pool.get()?;
        Ok(parents::table
            .find(parent_id)
            .first::<Parent>(&mut connection)?)
    }

    pub fn update(&self, parent_id: i32, updated_parent: UpdateParent) -> Result<Parent, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::update(parents::table.find(parent_id))
            .set(&updated_parent)
            .get_result::<Parent>(&mut connection)?)
    }

    pub fn delete(&self, parent_id: i32) -> Result<usize, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::delete(parents::table.find(parent_id)).execute(&mut connection)?)
    }
}
