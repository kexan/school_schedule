use std::env;

use diesel::prelude::*;
use tower_sessions_redis_store::fred::prelude::{
    Config, ConnectionConfig, PerformanceConfig, ReconnectPolicy,
};

pub type PostgresPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type PostgresConnection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type DieselError = r2d2::Error;
pub type RedisPool = tower_sessions_redis_store::fred::prelude::Pool;

pub fn establish_postgres_connection() -> PostgresPool {
    let postgres_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(postgres_url);
    diesel::r2d2::Pool::builder()
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

pub fn get_postgres_connection(pool: &PostgresPool) -> Result<PostgresConnection, DieselError> {
    pool.get()
}
