use std::fmt::Debug;

use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

use crate::schema::users::{self};
use axum_login::AuthUser;
use password_auth::{generate_hash, verify_password};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone, ToSchema)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip)]
    password: String,
    pub role: PermissionRole,
    pub full_name: Option<String>,
}

impl User {
    pub fn check_password(&self, creds: Credentials) -> bool {
        verify_password(creds.password, &self.password).is_ok()
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
pub struct NewUser {
    pub full_name: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUserWithPassword {
    username: String,
    password: String,
    full_name: Option<String>,
}

impl NewUserWithPassword {
    pub fn new(new_user: NewUser, creds: Credentials) -> Self {
        Self {
            username: creds.username,
            password: creds.password,
            full_name: new_user.full_name,
        }
    }
}
#[derive(Insertable, AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub role: Option<PermissionRole>,
    pub full_name: Option<String>,
}

#[derive(Debug, Clone, DbEnum, Serialize, Deserialize, ToSchema)]
#[db_enum(existing_type_path = "crate::schema::sql_types::PermissionRole")]
pub enum PermissionRole {
    User,
    Teacher,
    Director,
    Admin,
}

#[derive(Deserialize, Clone, ToSchema)]
pub struct Credentials {
    pub username: String,
    password: String,
}

impl Credentials {
    pub fn new(new_username: &str, new_password: &str) -> Self {
        Self {
            username: new_username.to_string(),
            password: generate_hash(new_password),
        }
    }
}
