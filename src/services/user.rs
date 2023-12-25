use crate::handlers;
use actix_web::web;

pub fn init_user_service(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/user")
            .service(handlers::user::get_all_users)
            .service(handlers::user::get_all_employers)
            .service(handlers::user::get_all_freelancers)
            .service(
                web::scope("/freelancer").service(
                    web::scope("/skills")
                        .route("/{id}", web::get().to(handlers::user::get_skills))
                        .route("", web::post().to(handlers::user::add_skills)),
                ),
            )
            .service(handlers::user::get_user)
            .service(handlers::user::register_user)
            .service(handlers::user::create_profile),
    );
}
