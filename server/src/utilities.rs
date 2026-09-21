use std::time::{SystemTime, UNIX_EPOCH};
use mongodb::bson::oid::ObjectId;

pub fn _id() -> String {
    ObjectId::new().to_string()
}

pub fn now_sec() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}