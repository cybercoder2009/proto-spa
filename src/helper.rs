use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const JWT_SECRET: &[u8] = b"super_secret_key_for_prototype";

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap_or_default()
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap_or(false)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User email/_id
    pub exp: usize,  // Expiration timestamp
}

pub fn create_token(email: &str) -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
    let expiration = now + (24 * 3600); // 24 hours

    let claims = Claims {
        sub: email.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)).unwrap_or_default()
}

pub fn verify_token(token: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .ok()
}
