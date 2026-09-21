pub mod login;

use axum::{routing, Router};
use crate::state::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/login", routing::post(login::post))
        .with_state(state)
}
