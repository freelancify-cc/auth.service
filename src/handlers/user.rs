use actix_web::{web, get, patch, delete, post, Responder};

use crate::state::AppState;

#[get("/")]
pub async fn get_all_users(state: web::Data<AppState>) -> impl Responder {
    web::Bytes::from_static(b"get all users") 
}

#[get("/{id}")]
pub async fn get_user(
    path: web::Path<i32>,
    state: web::Data<AppState>
) -> String {
    format!("get user by {}", path.into_inner()).to_string()
}

#[patch("/{id}")]
pub async fn update_user(
    path: web::Path<i32>,
    state: web::Data<AppState>
) -> String {
    format!("update user by {}", path.into_inner()).to_string()
}

#[delete("/{id}")]
pub async fn delete_user(
    path: web::Path<i32>,
    state: web::Data<AppState>
) -> String {
    format!("delete user by {}", path.into_inner()).to_string()
}

#[post("/")]
pub async fn create_user(
    state: web::Data<AppState>
) -> String {
     "Create user".to_string()
}
