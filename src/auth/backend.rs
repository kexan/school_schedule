use crate::{
    db::PostgresPool,
    error::AppError,
    models::user::{Credentials, User},
    schema::users::{self},
};
use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use diesel::prelude::*;
use tracing::info;

#[derive(Clone)]
pub struct AuthBackend {
    pool: PostgresPool,
}

impl AuthBackend {
    pub fn new(pool: PostgresPool) -> Self {
        AuthBackend { pool }
    }
}

#[async_trait]
impl AuthnBackend for AuthBackend {
    type User = User;
    type Credentials = Credentials;
    type Error = AppError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        info!("Trying to authenticate user {}", &creds.username);
        let mut connection = self.pool.get()?;
        let user_result: Option<User> = users::table
            .filter(users::username.eq(&creds.username))
            .first(&mut connection)
            .optional()?;

        Ok(user_result.filter(|user| user.check_password(creds)))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        info!("Trying get user with id {}", &user_id);
        let mut connection = self.pool.get()?;

        let user_result: Option<User> = users::table
            .filter(users::id.eq(user_id))
            .first(&mut connection)
            .optional()?;

        Ok(user_result)
    }
}
