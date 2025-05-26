use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    db::PostgresPool,
    error::AppError,
    models::document::{Document, NewDocument},
    schema::documents::{self, teacher_id},
};

#[derive(Clone)]
pub struct DocumentRepository {
    pool: PostgresPool,
}

impl DocumentRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, new_document: NewDocument) -> Result<Document, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::insert_into(documents::table)
            .values(new_document)
            .get_result::<Document>(&mut connection)?)
    }

    pub fn get(&self, document_id: Uuid) -> Result<Document, AppError> {
        let mut connection = self.pool.get()?;
        Ok(documents::table
            .find(document_id)
            .first::<Document>(&mut connection)?)
    }

    pub fn get_by_teacher_id(&self, id: i32) -> Result<Vec<Document>, AppError> {
        let mut connection = self.pool.get()?;
        Ok(documents::table
            .filter(teacher_id.eq(id))
            .load::<Document>(&mut connection)?)
    }

    pub fn delete(&self, document_id: Uuid) -> Result<usize, AppError> {
        let mut connection = self.pool.get()?;
        Ok(diesel::delete(documents::table.find(document_id)).execute(&mut connection)?)
    }
}
