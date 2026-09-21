pub mod handler;

use axum::{routing, Router};
use crate::state::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", routing::get(handler::ws_handler))
        .with_state(state)
}
