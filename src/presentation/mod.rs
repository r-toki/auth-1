use actix_web::web::ServiceConfig;

mod result;
mod users;

pub fn init(cfg: &mut ServiceConfig) {
    users::init(cfg);
}
