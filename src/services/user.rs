use actix_web::{web};
use crate::handlers;

pub fn init_user_service(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/user")
            .service(handlers::user::get_all_users)
            .service(handlers::user::get_user)
            .service(handlers::user::create_user)
            .service(handlers::user::delete_user)
            .service(handlers::user::update_user)
    );
}
