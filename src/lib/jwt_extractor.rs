use crate::lib::jwt::Claims;
use crate::lib::jwt::{decode_access_token, decode_refresh_token};
use actix_web::{http::header, FromRequest};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::future::Future;
use std::pin::Pin;

lazy_static! {
    static ref BEARER_REGEXP: Regex = Regex::new(r"^Bearer\s(.*)$").unwrap();
}

pub struct BearerToken(pub String);

impl FromRequest for BearerToken {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let token = extract_bearer_token(&req);
            match token {
                Some(token) => Ok(BearerToken(token)),
                None => Err(actix_web::error::ErrorForbidden("Forbidden")),
            }
        })
    }
}

pub struct AccessTokenDecoded(pub Option<Claims>);

impl FromRequest for AccessTokenDecoded {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            match extract_bearer_token(&req) {
                Some(token) => {
                    let claims = decode_access_token(&token);
                    match claims {
                        Ok(claims) => Ok(AccessTokenDecoded(Some(claims))),
                        Err(_) => Ok(AccessTokenDecoded(None)),
                    }
                }
                None => Ok(AccessTokenDecoded(None)),
            }
        })
    }
}

pub struct RefreshTokenDecoded(pub Option<Claims>);

impl FromRequest for RefreshTokenDecoded {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            match extract_bearer_token(&req) {
                Some(token) => {
                    let claims = decode_refresh_token(&token);
                    match claims {
                        Ok(claims) => Ok(RefreshTokenDecoded(Some(claims))),
                        Err(_) => Ok(RefreshTokenDecoded(None)),
                    }
                }
                None => Ok(RefreshTokenDecoded(None)),
            }
        })
    }
}

fn extract_bearer_token(req: &actix_web::HttpRequest) -> Option<String> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|authorization| {
            BEARER_REGEXP
                .captures(authorization)
                .and_then(|captures| captures.get(1))
        })
        .map(|v| v.as_str().to_owned())
}
