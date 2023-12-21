use crate::middleware;
use crate::schema::token::TokenClaims;
use crate::schema::user::LoginUserSchema;
use crate::{config, state::AppState};

use crate::models::user::User;

use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpResponse, Responder,
};
use actix_web::{HttpMessage, HttpRequest};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

#[post("/login")]
pub async fn authenticate(
    body: web::Json<LoginUserSchema>,
    state: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", body.email)
        .fetch_optional(state.get_db())
        .await
        .unwrap();

    let is_valid = query_result.as_ref().map_or(false, |user| {
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
    });

    if !is_valid {
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid email or password"}));
    }

    let user = query_result.unwrap();

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config::get("JWT_PRIVATE_KEY").as_bytes()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
}

#[get("/get_public_key")]
pub async fn get_public_key(_: web::Data<AppState>) -> impl Responder {
    let key = config::get("JWT_PUBLIC_KEY");
    let response = serde_json::json!({
        "status": "sucess",
        "public_key": key
    });

    return HttpResponse::Ok().json(response);
}

#[get("/userinfo")]
pub async fn get_userinfo(
    req: HttpRequest,
    _: web::Data<AppState>,
    _: middleware::auth::JwtMiddleware,
) -> impl Responder {
    let extension = req.extensions();
    let user_id = extension.get::<uuid::Uuid>().unwrap();

    log::info!("calling userinfo");
    let user_response = serde_json::json!({
        "status": "success",
        "sub": user_id
    });

    return HttpResponse::Ok().json(user_response);
}
