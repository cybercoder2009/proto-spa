use crate::helper::{create_token, verify_password};
use crate::structs::state::State as AppState;
use crate::structs::user::User;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PostReq {
    pub email: String,
    #[serde(alias = "password")]
    pub pass: String,
}

#[derive(Debug, Serialize)]
pub struct PostResp {
    pub token: String,
    pub email: String,
    pub role: String,
}

pub async fn post(
    State(state): State<AppState>,
    Json(payload): Json<PostReq>,
) -> impl IntoResponse {
    let collection = state.db.collection::<User>("users");
    log::info!("Login attempt for email: {}", payload.email);

    // Look up the hashed password from DB
    let user = match collection
        .find_one(doc! { "_id": &payload.email }, None)
        .await
    {
        Ok(Some(u)) => u,
        _ => {
            log::warn!("Login rejected: User {} not found", payload.email);
            return (
                StatusCode::UNAUTHORIZED,
                "Authentication failed: invalid user",
            )
                .into_response();
        }
    };

    // Securely compare the hash
    if !verify_password(&payload.pass, &user.pass) {
        log::warn!("Login rejected: Incorrect password for {}", payload.email);
        return (
            StatusCode::UNAUTHORIZED,
            "Authentication failed: incorrect password",
        )
            .into_response();
    }

    // Mint the token on success - subsequent requests don't need to query DB again
    let token = create_token(&user._id);
    log::info!("Login successful: {}", payload.email);

    let role = if user._id.contains("admin") {
        "admin"
    } else {
        "user"
    };

    (
        StatusCode::OK,
        Json(PostResp {
            token,
            email: user._id,
            role: role.to_string(),
        }),
    )
        .into_response()
}
