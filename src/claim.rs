use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// Token lifetime and Secret key are hardcoded for clarity
// const JWT_EXPIRATION_HOURS: i64 = 24;
const SECRET: &str = "SECRET";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub user_id: i64,
    pub username: String,
    pub permissions: Vec<String>,
    exp: i64,
}

impl Claims {
    pub fn new(user_id:i64, username: String, permissions: Vec<String>) -> Self {
        Self {
            user_id,
            username,
            permissions,
            exp: (Utc::now() + Duration::hours(dotenv!("TOKEN_DURATION_TIME_HOURS").parse::<i64>().unwrap())).timestamp(),
        }
    }
}

pub(crate) fn create_jwt(claims: Claims) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}

pub(crate) fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let decoding_key = DecodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}
