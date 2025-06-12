use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder, tower_sessions::SessionManagerLayer};
use tower_sessions_redis_store::{RedisStore, fred::prelude::Pool};

use crate::{
    auth::backend::AuthBackend,
    db::{PostgresPool, RedisPool},
};

pub mod backend;

pub fn get_auth_layer(
    pg_pool: PostgresPool,
    redis_pool: RedisPool,
) -> AuthManagerLayer<AuthBackend, RedisStore<Pool>> {
    let session_store = RedisStore::new(redis_pool);
    //TODO: SSL support
    let session_manager_layer = SessionManagerLayer::new(session_store).with_secure(false);
    let auth_backend = AuthBackend::new(pg_pool);
    AuthManagerLayerBuilder::new(auth_backend, session_manager_layer).build()
}
