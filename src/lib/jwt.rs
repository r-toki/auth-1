use crate::lib::config::CONFIG;
use chrono::{Duration, Utc};
use derive_new::new;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(new, Debug)]
pub struct Auth {
    pub user_id: String,
}

#[derive(new, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

#[derive(new, Debug, Serialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn generate_tokens(user_id: &str) -> anyhow::Result<Tokens, jsonwebtoken::errors::Error> {
    let access_exp = (Utc::now() + Duration::minutes(30)).timestamp();
    let access_claims = Claims::new(user_id.to_owned(), access_exp);
    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(CONFIG.access_token_secret.as_bytes()),
    )?;

    let refresh_exp = (Utc::now() + Duration::minutes(30)).timestamp();
    let refresh_claims = Claims::new(user_id.to_owned(), refresh_exp);
    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(CONFIG.refresh_token_secret.as_bytes()),
    )?;

    Ok(Tokens::new(access_token, refresh_token))
}

pub fn decode_access_token(token: &str) -> anyhow::Result<Claims, jsonwebtoken::errors::Error> {
    decode(
        token,
        &DecodingKey::from_secret(CONFIG.access_token_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|v| v.claims)
}

pub fn decode_refresh_token(token: &str) -> anyhow::Result<Claims, jsonwebtoken::errors::Error> {
    decode(
        token,
        &DecodingKey::from_secret(CONFIG.refresh_token_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|v| v.claims)
}
