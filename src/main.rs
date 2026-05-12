use axum::{Router, routing::post};
use flexi_logger::{Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming};
use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use std::fs::read_to_string;
use std::sync::Arc;
use tower_http::services::ServeDir;

use server::routes::login;
use server::routes::register;
use server::seeding;
use server::structs::config::Config;
use server::structs::state::State;

#[tokio::main]
async fn main() {
    // read config
    let config_str: String = read_to_string("config.toml").expect("err-reading-config");
    let config: Config = toml::from_str(&config_str).expect("err-parsing-config");

    // init logging
    let _logger = Logger::try_with_env_or_str("info")
        .expect("failed to parse logger env")
        .format(flexi_logger::detailed_format)
        .log_to_file(FileSpec::default().directory("logs").basename("server"))
        .duplicate_to_stderr(Duplicate::All)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(30),
        )
        .start()
        .expect("failed to initialize flexi_logger");

    // init db
    let opt: ClientOptions = ClientOptions::parse(config.mongo_binding)
        .await
        .expect("err-parsing-mongo-connection-string");
    let db: Database = Client::with_options(opt)
        .expect("err-connecting-mongo")
        .database(&config.mongo_db);

    // init users
    seeding::users(&db).await.expect("err-seeding-db");

    // init state
    let state: State = State { db: Arc::new(db) };

    // init server
    use axum::http::Method;
    use tower_http::cors::{Any, CorsLayer};

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/register", post(register::post))
        .route("/api/login", post(login::post))
        .fallback_service(ServeDir::new("./public"))
        .layer(cors)
        .with_state(state);
    println!("Server listening on: {}", config.binding);

    axum::serve(
        tokio::net::TcpListener::bind(&config.binding)
            .await
            .expect("err-binding-server"),
        app,
    )
    .await
    .expect("err-running-server");
}
