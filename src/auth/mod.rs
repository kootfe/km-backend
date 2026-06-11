pub mod jwt;
pub mod user;
pub mod extractor;

pub use jwt::*;
pub use user::*;
pub use extractor::*;

use crate::data::{KfResponse, app::*};

use actix_web::{HttpResponse, Responder, get, post, web};
use bcrypt::{DEFAULT_COST, hash, verify};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[post("/register")]
pub async fn register(
    _app_state: web::Data<AppState>,
    pool: web::Data<PgPool>,
    body: web::Json<AuthRequest>,
) -> impl Responder {
    let password_hash = match hash(&body.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(KfResponse::err("Failed to hash the password"));
        }
    };

    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)",
        body.username,
        password_hash
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(KfResponse::suc()),
        Err(sqlx::Error::Database(e)) if e.constraint() == Some("users_username_key") => {
            HttpResponse::Conflict().json(KfResponse::err("Username allready taken"))
        }
        Err(_) => HttpResponse::InternalServerError().json(KfResponse::err("Registration failed")),
    }
}

#[post("/login")]
pub async fn login(
    app_state: web::Data<AppState>,
    pool: web::Data<PgPool>,
    body: web::Json<AuthRequest>,
) -> impl Responder {
    let row = sqlx::query!(
        "select id, password_hash from users where username = $1",
        body.username
    )
    .fetch_optional(pool.get_ref())
    .await;

    match row {
        Ok(Some(u)) => {
            if verify(&body.password, &u.password_hash).unwrap_or(false) {
                match create_token(&u.id.to_string(), &app_state.jwt_secret) {
                    Ok(token) => HttpResponse::Ok().json(KfResponse::ok(token)),
                    Err(_) => {
                        HttpResponse::InternalServerError().json(KfResponse::err("token error"))
                    }
                }
            } else {
                HttpResponse::Unauthorized().json(KfResponse::err("user or password is wrong"))
            }
        }
        Ok(None) => HttpResponse::Unauthorized().json(KfResponse::err("user or password is wrong")),
        Err(_) => HttpResponse::InternalServerError().json(KfResponse::err("db error")),
    }
}

#[get("/me")]
pub async fn me(AuthUser(user): AuthUser) -> impl Responder {
    HttpResponse::Ok().json(KfResponse::ok(user))
}
