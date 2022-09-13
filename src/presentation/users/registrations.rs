use crate::lib::jwt::Tokens;
use crate::lib::jwt_extractor::AccessTokenDecoded;
use crate::module::Modules;
use crate::presentation;
use actix_web::{
    delete, post,
    web::{Data, Json, ServiceConfig},
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
    modules: Data<Modules>,
    form: Json<CreateForm>,
) -> presentation::Result<Json<Tokens>> {
    let tokens = modules
        .auth_service
        .sign_up(&form.email, &form.password)
        .await?;

    Ok(Json(tokens))
}

#[delete("/users/registrations")]
async fn destroy(
    modules: Data<Modules>,
    access_token_decoded: AccessTokenDecoded,
) -> presentation::Result<Json<()>> {
    modules
        .auth_service
        .delete_user(access_token_decoded.into())
        .await?;

    Ok(Json(()))
}
