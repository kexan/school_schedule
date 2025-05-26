use axum::extract::FromRef;
use tracing::{info, warn};

use crate::{
    AppState,
    error::AppError,
    logic::repositories::parent_repository::ParentRepository,
    models::parent::{NewParent, Parent, UpdateParent},
};

#[derive(Clone)]
pub struct ParentService {
    parent_repository: ParentRepository,
}

impl ParentService {
    pub fn new(parent_repository: ParentRepository) -> Self {
        Self { parent_repository }
    }

    pub fn create(&self, new_parent: NewParent) -> Result<Parent, AppError> {
        let parent = self.parent_repository.create(new_parent)?;
        info!("Successfully created parent with ID {}", parent.id);
        Ok(parent)
    }

    pub fn get(&self, parent_id: i32) -> Result<Parent, AppError> {
        let parent = self.parent_repository.get(parent_id)?;
        info!("Parent with ID {} successfully get", parent_id);
        Ok(parent)
    }

    pub fn update(&self, parent_id: i32, update_parent: UpdateParent) -> Result<Parent, AppError> {
        let updated_parent = self.parent_repository.update(parent_id, update_parent)?;
        info!("Parent with ID {} was successfully updated", parent_id);
        Ok(updated_parent)
    }

    pub fn delete(&self, parent_id: i32) -> Result<bool, AppError> {
        let deleted_count = self.parent_repository.delete(parent_id)?;

        if deleted_count > 0 {
            info!("Parent with ID {} was successfully deleted", parent_id);
            Ok(true)
        } else {
            warn!("Parent with ID {} not found", parent_id);
            Ok(false)
        }
    }
}

impl FromRef<AppState> for ParentService {
    fn from_ref(state: &AppState) -> Self {
        state.services.parent_service.clone()
    }
}
