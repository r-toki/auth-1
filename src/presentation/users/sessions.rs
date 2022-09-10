use crate::lib::jwt_extractor::{AccessTokenDecoded, BearerToken, RefreshTokenDecoded};
use crate::presentation::result::Result;
use actix_web::{
    delete, patch, post,
    web::{Json, ServiceConfig},
    Responder,
};
use serde::Deserialize;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
    cfg.service(update);
    cfg.service(destroy);
}

#[derive(Debug, Deserialize)]
struct Create {
    email: String,
    password: String,
}

#[post("/users/sessions")]
async fn create(form: Json<Create>) -> Result<impl Responder> {
    Ok(Json(()))
}

#[patch("/users/sessions")]
async fn update(
    bearer_token: BearerToken,
    refresh_token_decoded: RefreshTokenDecoded,
) -> Result<impl Responder> {
    Ok(Json(()))
}

#[delete("/users/sessions")]
async fn destroy(access_token_decoded: AccessTokenDecoded) -> Result<impl Responder> {
    Ok(Json(()))
}