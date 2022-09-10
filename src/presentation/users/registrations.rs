use crate::lib::jwt_extractor::AccessTokenDecoded;
use crate::presentation;
use actix_web::{
    delete, post,
    web::{Json, ServiceConfig},
    Responder,
};
use serde::Deserialize;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
    cfg.service(destroy);
}

#[derive(Debug, Deserialize)]
struct CreateForm {
    email: String,
    password: String,
}

#[post("/users/registrations")]
async fn create(form: Json<CreateForm>) -> presentation::Result<impl Responder> {
    Ok(Json(()))
}

#[delete("/users/registrations")]
async fn destroy(access_token_decoded: AccessTokenDecoded) -> presentation::Result<impl Responder> {
    Ok(Json(()))
}
