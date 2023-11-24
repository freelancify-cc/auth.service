use crate::models::user::{User, UserProfile, UserInformationModel, filter_user_record, filter_userprofile_record};
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
use uuid::Uuid;

#[get("/")]
pub async fn get_all_users(state: web::Data<AppState>) -> impl Responder {
    let users_query = sqlx::query_as!(UserInformationModel, 
            "SELECT users.id, users.email,
                    userprofiles.username, userprofiles.first_name, userprofiles.second_name, userprofiles.profile_picture_url, userprofiles.date_of_birth,
                    userroles.role_name
             FROM users 
             JOIN userprofiles
             ON users.id = userprofiles.user_id
             JOIN userroles
             ON users.user_role_id = userroles.id")
        .fetch_all(state.get_db())
        .await
        .unwrap();

    match users_query {
        users => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "users":  &users
            })});

            return HttpResponse::Ok().json(user_response);
        }
    }
}

#[get("/{id}")]
pub async fn get_user(
    path: web::Path<uuid::Uuid>,
    state: web::Data<AppState>
) -> impl Responder {
    let user_id = Uuid::parse_str(path.into_inner().to_string().as_str());
    match user_id {
        Ok(id) => {
            let user_query = sqlx::query_as!(UserInformationModel,
                "SELECT users.id, users.email,
                        userprofiles.username, userprofiles.first_name, userprofiles.second_name, userprofiles.profile_picture_url, userprofiles.date_of_birth,
                        userroles.role_name
                    FROM users 
                    JOIN userprofiles
                    ON users.id = userprofiles.user_id
                    JOIN userroles
                    ON users.user_role_id = userroles.id
                    WHERE users.id = $1", id)
                .fetch_one(state.get_db())
                .await
                .unwrap();

            match user_query {
                user => {
                    let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                        "users":  &user
                    })});
        
                    return HttpResponse::Ok().json(user_response);
                }
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
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
