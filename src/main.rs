use actix_web::{
    App, HttpServer,
    http,
    web,
    HttpResponse, middleware::Logger,
};
use actix_cors::Cors;
use state::AppState;

mod state;
mod config;
mod database;
mod services;
mod models;
mod handlers;
mod schema;
mod middleware;

async fn say_hello(state: web::Data<AppState>) -> String {
    state.get_app_name().to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init();

    if config::get("BUILD") == "dev" {
        std::env::set_var("RUST_LOG", "debug");
    } 
    env_logger::init();
    println!("[!] Starting server on {}:{}", config::get("HOST"), config::get("PORT"));

    let state = state::AppState::new().await;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("0.0.0.0")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);        

         App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(cors)
            //.wrap(Logger::default())
            .service(web::scope("/api")
                .configure(services::user::init_user_service)
                .configure(services::auth::init_auth_service))
            .route(
                "/",
                web::get().to(say_hello),
            )
    })
    .bind(("0.0.0.0", config::get("PORT").parse::<u16>().unwrap()))?
    .run()
    .await
}
