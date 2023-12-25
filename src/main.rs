use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, App, HttpResponse, HttpServer};
use state::AppState;

mod config;
mod database;
mod handlers;
mod middleware;
mod models;
mod schema;
mod services;
mod state;

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
    println!(
        "[!] Starting server on {}:{}",
        config::get("HOST"),
        config::get("PORT")
    );

    let state = state::AppState::new().await;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("127.0.0.1:3000")
            .allowed_origin("127.0.0.1:8001")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .configure(services::user::init_user_service)
                    .configure(services::auth::init_auth_service),
            )
            .route("/", web::get().to(say_hello))
    })
    .bind(("0.0.0.0", config::get("PORT").parse::<u16>().unwrap()))?
    .run()
    .await
}
