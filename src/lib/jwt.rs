use crate::lib::config::CONFIG;
use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn encode_access_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = Utc::now() + Duration::minutes(30);
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: exp.timestamp() as usize,
    };
    let secret = &EncodingKey::from_secret(CONFIG.access_token_secret.as_bytes());
    encode(&Header::default(), &claims, secret)
}

pub fn decode_access_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = &DecodingKey::from_secret(CONFIG.access_token_secret.as_bytes());
    decode::<Claims>(token, secret, &Validation::default()).map(|data| data.claims)
}

pub fn encode_refresh_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = Utc::now() + Duration::weeks(2);
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: exp.timestamp() as usize,
    };
    let secret = &EncodingKey::from_secret(CONFIG.refresh_token_secret.as_bytes());
    encode(&Header::default(), &claims, secret)
}

pub fn decode_refresh_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = &DecodingKey::from_secret(CONFIG.refresh_token_secret.as_bytes());
    decode::<Claims>(token, secret, &Validation::default()).map(|data| data.claims)
}
