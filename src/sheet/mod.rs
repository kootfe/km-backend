use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{auth::AuthUser, data::KfResponse};

#[derive(Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub description: Option<String>,
    pub schema: serde_json::Value,
    pub is_public: Option<bool>,
}

#[post("/")]
pub async fn create_template(
    AuthUser(user): AuthUser,
    pool: web::Data<PgPool>,
    body: web::Json<CreateTemplateRequest>,
) -> impl Responder {
    let result = sqlx::query!(
    "insert into char_sheet_templates (owner_id, name, description, schema, is_public) values ($1, $2, $3, $4, $5) returning id",
    user.id,
    body.name,
    body.description,
    body.schema,
    body.is_public.unwrap_or(false)
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(row) => HttpResponse::Ok().json(KfResponse::ok(row.id)),
        Err(_) => {
            HttpResponse::InternalServerError().json(KfResponse::err("failed to create template"))
        }
    }
}

#[get("/")]
pub async fn list_templates(pool: web::Data<PgPool>) -> impl Responder {
    let rows = sqlx::query!("select id, name, description, is_public, created_at from char_sheet_templates where is_public = true")
        .fetch_all(pool.get_ref()).await;
    match rows {
        Ok(rows) => {
            let templates: Vec<serde_json::Value> = rows
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "id": r.id,
                        "name": r.name,
                        "description": r.description,
                        "is_public": r.is_public,
                        "created_at": r.created_at
                    })
                })
                .collect();
            HttpResponse::Ok().json(KfResponse::ok(templates))
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(KfResponse::err("failed to fetch templates"))
        }
    }
}

#[get("/{id}")]
pub async fn get_template(AuthUser(user): AuthUser, pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();
    let row = sqlx::query!("select * from char_sheet_templates where id = $1", id)
        .fetch_optional(pool.get_ref())
        .await;

    match row {
        Ok(Some(r)) => {
            if !r.is_public && user.id != r.owner_id {
                return HttpResponse::Unauthorized().json(KfResponse::err("No permision."));
            }
            HttpResponse::Ok().json(KfResponse::ok(serde_json::json!({
            "id": r.id,
            "name": r.name,
            "description": r.description,
            "schema": r.schema,
            "is_public": r.is_public,
            "created_at": r.created_at,
        })))
        }
        Ok(None) => HttpResponse::NotFound().json(KfResponse::err("temlate not found")),
        Err(_) => HttpResponse::InternalServerError().json(KfResponse::err("db error")),
    }
}
