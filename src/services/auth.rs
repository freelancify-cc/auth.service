use crate::handlers;
use actix_web::web;

pub fn init_auth_service(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/auth")
            .service(handlers::auth::authenticate));
}
