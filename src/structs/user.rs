use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub _id: String,
    pub name: String,
    pub pass: String,
}
