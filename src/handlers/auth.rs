use actix_web::{web, post};

use crate::state::AppState;
use crate::models::user::loginUserSchema;

#[post("/login")]
pub async fn authenticate(
    body: web::Json<>
    state: web::Data<AppState>
) -> String {
     
}
