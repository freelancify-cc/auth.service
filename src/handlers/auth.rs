use actix_web::{web, post};

use crate::state::AppState;
use crate::models::user::LoginUserSchema;

use sqlx::Row;

#[post("/login")]
pub async fn authenticate(
    body: web::Json<LoginUserSchema>,
    state: web::Data<AppState>
) -> String {
    let query_result = sqlx::query_as!("SELECT email, username, password FROM email = $1", body.email);
}
