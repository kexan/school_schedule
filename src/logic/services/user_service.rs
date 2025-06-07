use axum::extract::FromRef;
use tracing::info;

use crate::{
    AppState,
    error::AppError,
    logic::repositories::user_repository::UserRepository,
    models::user::{Credentials, NewUser, NewUserWithPassword, UpdateUser, User},
};

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub fn create(&self, new_user: NewUser, creds: Credentials) -> Result<User, AppError> {
        let new_user = NewUserWithPassword::new(new_user, creds);
        let new_user = self.user_repository.create(new_user)?;
        info!(
            "Successfully created user with ID {} and username {}",
            new_user.id, new_user.username
        );
        Ok(new_user)
    }

    pub fn get(&self, user_id: i32) -> Result<User, AppError> {
        let user = self.user_repository.get(user_id)?;
        info!("Successfully get user with ID {}", user_id);
        Ok(user)
    }

    pub fn update(&self, user_id: i32, update_user: UpdateUser) -> Result<User, AppError> {
        let user = self.user_repository.update(user_id, update_user)?;
        info!("Successfully updated user with ID {}", user_id);
        Ok(user)
    }

    pub fn delete(&self, user_id: i32) -> Result<bool, AppError> {
        let deleted_count = self.user_repository.delete(user_id)?;
        if deleted_count > 0 {
            info!("Deleted user with ID {}", user_id);
            Ok(true)
        } else {
            info!("User with ID {} not found", user_id);
            Ok(false)
        }
    }
}

impl FromRef<AppState> for UserService {
    fn from_ref(state: &AppState) -> Self {
        state.services.user_service.clone()
    }
}
