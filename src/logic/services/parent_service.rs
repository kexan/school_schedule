use tracing::{info, warn};

use crate::{
    db::{self, PostgresPool},
    error::AppError,
    logic::repositories::parent_repository::ParentRepository,
    models::parent::{NewParent, Parent, UpdateParent},
};

pub struct ParentService;

impl ParentService {
    pub fn create(postgres_pool: &PostgresPool, new_parent: NewParent) -> Result<Parent, AppError> {
        let parent = db::with_connection(postgres_pool, |connection| {
            ParentRepository::create(connection, new_parent)
        })?;
        info!("Successfully created parent with ID {}", parent.id);
        Ok(parent)
    }

    pub fn get(postgres_pool: &PostgresPool, parent_id: i32) -> Result<Parent, AppError> {
        let parent = db::with_connection(postgres_pool, |connection| {
            ParentRepository::get(connection, parent_id)
        })?;
        info!("Parent with ID {} successfully get", parent_id);
        Ok(parent)
    }

    pub fn update(
        postgres_pool: &PostgresPool,
        parent_id: i32,
        update_parent: UpdateParent,
    ) -> Result<Parent, AppError> {
        let updated_parent = db::with_connection(postgres_pool, |connection| {
            ParentRepository::update(connection, parent_id, update_parent)
        })?;
        info!("Parent with ID {} was successfully updated", parent_id);
        Ok(updated_parent)
    }

    pub fn delete(postgres_pool: &PostgresPool, parent_id: i32) -> Result<bool, AppError> {
        let deleted_count = db::with_connection(postgres_pool, |connection| {
            ParentRepository::delete(connection, parent_id)
        })?;

        if deleted_count > 0 {
            info!("Parent with ID {} was successfully deleted", parent_id);
            Ok(true)
        } else {
            warn!("Parent with ID {} not found", parent_id);
            Ok(false)
        }
    }
}
