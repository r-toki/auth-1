use crate::lib::config::CONFIG;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub email: String,
}

#[derive(Debug)]
pub struct NewForAccessTokenInput {
    pub id: String,
    pub email: String,
}

#[derive(Debug)]
pub struct NewForRefreshTokenInput {
    pub id: String,
    pub email: String,
}

impl Claims {
    pub fn new_for_access_token(input: NewForAccessTokenInput) -> Self {
        let exp = Utc::now() + Duration::minutes(30);
        Claims {
            sub: input.id,
            exp: exp.timestamp() as usize,
            email: input.email,
        }
    }

    pub fn new_for_refresh_token(input: NewForRefreshTokenInput) -> Self {
        let exp = Utc::now() + Duration::weeks(2);
        Claims {
            sub: input.id,
            exp: exp.timestamp() as usize,
            email: input.email,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn create_tokens(id: &str, email: &str) -> anyhow::Result<Tokens> {
    let access_token_claims = Claims::new_for_access_token(NewForAccessTokenInput {
        id: id.to_owned(),
        email: email.to_owned(),
    });
    let access_token = encode_access_token(access_token_claims)?;

    let refresh_token_claims = Claims::new_for_refresh_token(NewForRefreshTokenInput {
        id: id.to_owned(),
        email: email.to_owned(),
    });
    let refresh_token = encode_refresh_token(refresh_token_claims)?;

    Ok(Tokens {
        access_token,
        refresh_token,
    })
}

fn encode_access_token(claims: Claims) -> anyhow::Result<String, jsonwebtoken::errors::Error> {
    let secret = &EncodingKey::from_secret(CONFIG.access_token_secret.as_bytes());
    encode(&Header::default(), &claims, secret)
}

fn encode_refresh_token(claims: Claims) -> anyhow::Result<String, jsonwebtoken::errors::Error> {
    let secret = &EncodingKey::from_secret(CONFIG.refresh_token_secret.as_bytes());
    encode(&Header::default(), &claims, secret)
}

pub fn decode_access_token(token: &str) -> anyhow::Result<Claims, jsonwebtoken::errors::Error> {
    let secret = &DecodingKey::from_secret(CONFIG.access_token_secret.as_bytes());
    decode::<Claims>(token, secret, &Validation::default()).map(|data| data.claims)
}

pub fn decode_refresh_token(token: &str) -> anyhow::Result<Claims, jsonwebtoken::errors::Error> {
    let secret = &DecodingKey::from_secret(CONFIG.refresh_token_secret.as_bytes());
    decode::<Claims>(token, secret, &Validation::default()).map(|data| data.claims)
}
