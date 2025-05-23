use std::env;

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use tower_sessions_redis_store::fred::prelude::{
    Config, ConnectionConfig, PerformanceConfig, ReconnectPolicy,
};

use crate::error::AppError;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;
pub type PostgresConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type DieselError = r2d2::Error;
pub type RedisPool = tower_sessions_redis_store::fred::prelude::Pool;

pub fn establish_postgres_connection() -> PostgresPool {
    let postgres_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_manager = ConnectionManager::<PgConnection>::new(postgres_url);
    Pool::builder()
        .build(connection_manager)
        .expect("Failed to create Postgres pool")
}

pub async fn establish_redis_connection() -> RedisPool {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    RedisPool::new(
        Config::from_url_centralized(redis_url.as_str()).unwrap(),
        Some(PerformanceConfig::default()),
        Some(ConnectionConfig::default()),
        Some(ReconnectPolicy::default()),
        6,
    )
    .expect("Failed to create Redis pool")
}

pub fn run_db_migrations(pool: &PostgresPool) {
    let mut connection = pool.get().expect("Failed to get connection for migrations");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}

pub fn with_connection<T, E, F>(pool: &PostgresPool, f: F) -> Result<T, AppError>
where
    F: FnOnce(&mut PooledConnection<ConnectionManager<diesel::PgConnection>>) -> Result<T, E>,
    E: Into<AppError>,
{
    let mut connection = pool.get()?;
    f(&mut connection).map_err(Into::into)
}
