use crate::lib::jwt::Tokens;
use crate::lib::jwt_extractor::{AccessTokenDecoded, BearerToken, RefreshTokenDecoded};
use crate::module::Modules;
use crate::presentation;
use actix_web::{
    delete, patch, post,
    web::{Data, Json, ServiceConfig},
};
use serde::Deserialize;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
    cfg.service(update);
    cfg.service(destroy);
}

#[derive(Debug, Deserialize)]
struct CreateForm {
    email: String,
    password: String,
}

#[post("/users/sessions")]
async fn create(
    modules: Data<Modules>,
    form: Json<CreateForm>,
) -> presentation::Result<Json<Tokens>> {
    let tokens = modules
        .auth_service
        .sign_in(&form.email, &form.password)
        .await?;

    Ok(Json(tokens))
}

#[patch("/users/sessions")]
async fn update(
    refresh_token_decoded: RefreshTokenDecoded,
    bearer_token: BearerToken,
    modules: Data<Modules>,
) -> presentation::Result<Json<Tokens>> {
    let bearer_token: String = bearer_token.into();
    let tokens = modules
        .auth_service
        .refresh(refresh_token_decoded.into(), &bearer_token)
        .await?;

    Ok(Json(tokens))
}

#[delete("/users/sessions")]
async fn destroy(
    access_token_decoded: AccessTokenDecoded,
    modules: Data<Modules>,
) -> presentation::Result<Json<()>> {
    modules
        .auth_service
        .sign_out(access_token_decoded.into())
        .await?;

    Ok(Json(()))
}
