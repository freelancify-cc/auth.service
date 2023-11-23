use crate::models::user::{User, UserProfile, filter_user_record, filter_userprofile_record };
use crate::schema::user;

use crate::{
    AppState,
};
use actix_web::{
    get, post, web, HttpResponse, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::Row;

#[get("/")]
pub async fn get_all_users(state: web::Data<AppState>) -> impl Responder {
    web::Bytes::from_static(b"get all users") 
    //let query_result = sqlx::query_as!()
}

#[get("/{id}")]
pub async fn get_user(
    path: web::Path<i32>,
    state: web::Data<AppState>
) -> String {
    format!("get user by {}", path.into_inner()).to_string()
}

#[post("/register")]
pub async fn register_user(
    body: web::Json<user::CreateUserSchema>,
    state: web::Data<AppState>
) -> impl Responder {
    let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
        .bind(body.email.to_owned())
        .fetch_one(state.get_db())
        .await
        .unwrap()
        .get(0);

    if exists {
        return HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "User with that email already exists"}),
        );
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users (email, password, user_role_id) VALUES ($1, $2, $3) RETURNING *",
        body.email.to_string().to_lowercase(), 
        hashed_password,
        body.role
    )
    .fetch_one(state.get_db())
    .await;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user":  filter_user_record(&user)
            })});

            return HttpResponse::Ok().json(user_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    } 
}
 
#[post("/create_profile")]
pub async fn create_profile(
    body: web::Json<user::CreateUserProfileSchema>,
    state: web::Data<AppState>
) -> impl Responder {
    let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)")
        .bind(body.id)
        .fetch_one(state.get_db())
        .await
        .unwrap()
        .get(0);

    if !exists {
        return HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "User with that id does not exist"}),
        );
    }
    let query_result = sqlx::query_as!(
        UserProfile,
        "INSERT INTO userprofiles (username, first_name, second_name, date_of_birth, profile_picture_url, user_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        body.username.to_string().to_lowercase(), 
        body.first_name.to_string().to_lowercase(), 
        body.second_name.to_string().to_lowercase(), 
        body.date_of_birth.to_string(),
        body.profile_picture_url,
        body.id 
    )
    .fetch_one(state.get_db())
    .await;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "profile":  filter_userprofile_record(&user)
            })});

            return HttpResponse::Ok().json(user_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    } 
}
