use actix_web::web;

use crate::controllers::create_trainer::create_trainer;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_trainer);

    // Add more routes here...
}
