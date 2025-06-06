use std::fmt::Debug;

use diesel::prelude::*;

use crate::{
    auth::permissions::PermissionRole,
    schema::users::{self},
};
use axum_login::AuthUser;
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone, ToSchema)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip)]
    password: String,
    pub role: PermissionRole,
}

impl User {
    pub fn check_password(&self, password: &String) -> bool {
        verify_password(password, &self.password).is_ok()
    }
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

#[derive(Insertable, AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub role: Option<PermissionRole>,
}
