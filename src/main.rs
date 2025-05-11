use axum_login::tower_sessions::SessionManagerLayer;
use dotenvy::dotenv;
use school_schedule::{db, open_api::ApiDoc};
use std::io::Error;
use tokio::net::TcpListener;
use tower_sessions_redis_store::{RedisStore, fred::prelude::ClientLike};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let redis_pool = db::establish_redis_connection().await;
    redis_pool.connect_pool();
    redis_pool
        .wait_for_connect()
        .await
        .expect("Failed to create Redis connection");
    let session_store = RedisStore::new(redis_pool);
    //TODO: SSL support
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    let postgre_pool = db::establish_postgres_connection();
    let connection = db::get_postgres_connection(&postgre_pool);

    let (router, open_api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .with_state(postgre_pool)
        .split_for_parts();
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, router).await
}
