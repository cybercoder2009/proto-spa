use axum::extract::DefaultBodyLimit;
use axum::http::Method;
use std::net::SocketAddr;
use std::sync::Arc;
use log;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use server::config::Config;
use server::db::Db;
use server::logger;
use server::state::AppState;
use server::routes;

#[tokio::main]
async fn main() {

    let config = Arc::new(Config::load());

    logger::init();

    // Connect to MongoDB
    let db = Db::connect(&config.mongo_uri, &config.mongo_db).await.expect("Failed to connect to MongoDB");
    log::info!("db={}/{}", config.mongo_uri, config.mongo_db);
    if let Err(e) = db.seeding().await {
        log::warn!("db-error={}", e);
    }

    let state = AppState {
        config: Arc::clone(&config),
        db,
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);

    let serve_dir = ServeDir::new(&config.public).append_index_html_on_directories(true);
    let body_limit = config.max_body_mb.max(16) * 1024 * 1024;

    let app = routes::routes(state)
        .fallback_service(serve_dir)
        .layer(DefaultBodyLimit::max(body_limit))
        .layer(cors);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .unwrap_or_else(|_| SocketAddr::from(([0, 0, 0, 0], config.port)));

    log::info!("server=http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
