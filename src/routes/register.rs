use crate::helper::hash_password;
use crate::structs::state::State as AppState;
use crate::structs::user::User;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use mongodb::bson::doc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostReq {
    pub email: String,
    pub name: String,
    #[serde(alias = "password")]
    pub pass: String,
}

pub async fn post(
    State(state): State<AppState>,
    Json(payload): Json<PostReq>,
) -> impl IntoResponse {
    let collection = state.db.collection::<User>("users");
    log::info!("Registration attempt for: {}", payload.email);

    // Pre-check to see if already in database
    if let Ok(Some(_)) = collection
        .find_one(doc! { "_id": &payload.email }, None)
        .await
    {
        log::warn!("Registration denied: {} already in database", payload.email);
        return (
            StatusCode::BAD_REQUEST,
            "User with this email already exists",
        )
            .into_response();
    }

    let new_user = User {
        _id: payload.email.clone(),
        name: payload.name,
        pass: hash_password(&payload.pass),
    };

    match collection.insert_one(new_user, None).await {
        Ok(_) => {
            log::info!("User {} registered successfully", payload.email);
            (StatusCode::CREATED, "Registration successful").into_response()
        }
        Err(e) => {
            log::error!("Registration database failure for {}: {:?}", payload.email, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal db error").into_response()
        }
    }
}
