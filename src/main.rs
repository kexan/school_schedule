use dotenvy::dotenv;
use school_schedule::{
    AppState,
    auth::{self},
    db, handlers,
    logic::services,
    open_api::ApiDoc,
};
use std::io::Error;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tower_sessions_redis_store::fred::prelude::ClientLike;
use tracing::info;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();
    dotenv().ok();
    let redis_pool = db::establish_redis_connection().await;
    redis_pool.connect_pool();
    redis_pool
        .wait_for_connect()
        .await
        .expect("Failed to create Redis connection");

    let postgres_pool = db::establish_postgres_connection();
    db::run_db_migrations(&postgres_pool);
    let auth_layer = auth::get_auth_layer(postgres_pool.clone(), redis_pool);
    let services = services::init_app_services(postgres_pool);
    let state = AppState { services };

    let (router, open_api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1/auth", handlers::auth_handler::router())
        .nest("/api/v1/users", handlers::user_handler::router())
        .nest("/api/v1/students", handlers::student_handler::router())
        .nest(
            "/api/v1/student_groups",
            handlers::student_group_handler::router(),
        )
        .nest("/api/v1/parents", handlers::parent_handler::router())
        .nest("/api/v1/teachers", handlers::teacher_handler::router())
        .nest("/api/v1/lessons", handlers::lesson_handler::router())
        .nest(
            "/api/v1/attendances",
            handlers::attendances_handler::router(),
        )
        .layer(TraceLayer::new_for_http())
        .layer(auth_layer)
        .with_state(state)
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger").url("/apidoc/openapi.json", open_api));

    let listener = TcpListener::bind("0.0.0.0:1234").await?;
    info!("Server started on {}", listener.local_addr()?);
    axum::serve(listener, router).await
}

fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("school_schedule=trace,tower_http=warn"))
                .unwrap(),
        )
        .init();
}
