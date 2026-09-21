pub mod rest;
pub mod ws;

use axum::Router;
use crate::state::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .nest("/rest", rest::routes(state.clone()))
        .nest("/ws", ws::routes(state))
}