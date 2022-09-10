mod registrations;
mod sessions;

use actix_web::web::ServiceConfig;

pub fn init(cfg: &mut ServiceConfig) {
    registrations::init(cfg);
    sessions::init(cfg);
}
