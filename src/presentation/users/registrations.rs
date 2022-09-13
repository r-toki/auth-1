use crate::context::Context;
use crate::lib::jwt::Tokens;
use crate::lib::jwt_extractor::AccessTokenDecoded;
use crate::presentation;
use actix_web::{
    delete, post,
    web::{Data, Json, ServiceConfig},
    Responder,
};
use serde::Deserialize;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
    cfg.service(destroy);
}

#[derive(Debug, Deserialize)]
struct CreateForm {
    pub email: String,
    pub password: String,
}

#[post("/users/registrations")]
async fn create(
    context: Data<Context>,
    form: Json<CreateForm>,
) -> presentation::Result<Json<Tokens>> {
    let tokens = context
        .auth_service
        .sign_up(&form.email, &form.password)
        .await?;

    Ok(Json(tokens))
}

#[delete("/users/registrations")]
async fn destroy(access_token_decoded: AccessTokenDecoded) -> presentation::Result<impl Responder> {
    Ok(Json(()))
}
