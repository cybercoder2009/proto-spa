use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use common::rest::login::{PostReq, PostResp};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use crate::constants::JWT_SECRET;
use crate::db::{COL_USERS, user::User};
use crate::state::AppState;
use crate::utilities;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    #[serde(flatten)]
    pub data: PostResp,
}

pub async fn post(
    State(state): State<AppState>,
    Json(req): Json<PostReq>,
) -> Result<String, StatusCode> {
    if req.username.trim().is_empty() || req.password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let collection = state.db.client.database(&state.db.db).collection::<User>(COL_USERS);

    let user = collection
        .find_one(doc! { "username": &req.username }, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::FORBIDDEN)?;

    let is_valid = bcrypt::verify(&req.password, &user.password).unwrap_or(false);
    if !is_valid {
        return Err(StatusCode::FORBIDDEN);
    }

    let exp = (utilities::now_sec() + 7 * 24 * 3600) as usize; // 7 days expiration
    let claims = Claims {
        exp,
        data: PostResp {
            username: user.username,
        },
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(token)
}
