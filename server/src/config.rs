use serde::Deserialize;
use crate::constants::CONFIG;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub public: String,
    pub max_body_mb: usize,

    // Database
    pub mongo_uri: String,
    pub mongo_db: String,
}

impl Config {
    pub fn load() -> Self {
        let config_str = std::fs::read_to_string(CONFIG).expect(&format!("{} error 0", CONFIG));
        let config = toml::from_str::<Config>(&config_str).expect(&format!("{} error 1", CONFIG));
        config
    }
}
