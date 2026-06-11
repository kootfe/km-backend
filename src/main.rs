pub mod auth;
pub mod data;
pub mod sheet;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

use crate::data::AppState;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"Vallaha\": 2}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to conntect to Postgres");

    let app_state = AppState::new(secret.to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(app_state.clone()))
            .service(root).service(
            web::scope("/auth")
                .service(auth::register)
                .service(auth::login)
                .service(auth::me)
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
