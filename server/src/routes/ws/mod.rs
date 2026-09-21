use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::{header, HeaderMap, StatusCode},
    response::Response,
    routing, Router,
};
use common::ws::ServerMsg;
use futures_util::{SinkExt, StreamExt};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

use crate::constants::JWT_SECRET;
use crate::routes::rest::login::Claims;
use crate::state::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", routing::get(ws_handler))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
pub struct WsAuthQuery {
    pub token: Option<String>,
}

fn verify_token(token: &str) -> Result<Claims, StatusCode> {
    let key = DecodingKey::from_secret(JWT_SECRET);
    let mut validation = Validation::default();
    validation.validate_exp = true;

    decode::<Claims>(token, &key, &validation)
        .map(|data| data.claims)
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(_state): State<AppState>,
    Query(query): Query<WsAuthQuery>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    let token = query
        .token
        .or_else(|| {
            headers
                .get(header::AUTHORIZATION)
                .and_then(|val| val.to_str().ok())
                .and_then(|val| val.strip_prefix("Bearer ").map(|s| s.to_string()))
        })
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = verify_token(&token)?;

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, claims)))
}

async fn handle_socket(socket: WebSocket, claims: Claims) {
    let username = claims.data.username;
    log::info!("ws connected: user={}", username);

    let (mut sender, mut receiver) = socket.split();

    // 握手成功后，服务端主动推送一条 echo 消息
    let echo_msg = ServerMsg::Echo(format!("Welcome {}, WebSocket connected!", username));
    if let Ok(msg_text) = serde_json::to_string(&echo_msg) {
        if sender.send(Message::Text(msg_text.into())).await.is_err() {
            return;
        }
    }

    // 监听连接状态与心跳/关闭
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Ping(payload) => {
                if sender.send(Message::Pong(payload)).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => {
                log::info!("ws closed by client: user={}", username);
                break;
            }
            _ => {}
        }
    }

    log::info!("ws disconnected: user={}", username);
}
