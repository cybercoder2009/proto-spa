use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub binding: String,
    pub mongo_binding: String,
    pub mongo_db: String,
}
