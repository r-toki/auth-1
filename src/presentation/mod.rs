use actix_web::web::ServiceConfig;

mod result;
mod users;

use result::Result;

pub fn init(cfg: &mut ServiceConfig) {
    users::init(cfg);
}
