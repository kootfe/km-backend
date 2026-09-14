#![allow(unused_variables)]

use actix_web::{HttpResponse, Responder, get, post, web};

use crate::{
    app::KM, baker, data::form::{LoginForm, RegisterForm}, db::users::get_user_count, http::response::{ApiErrorE, ApiResponse}, services::user::{login_user, register_user}, web_pages::{index_page, login_page, register_page}
};

#[get("/")]
pub async fn index(km: web::Data<KM>) -> impl Responder {
    // I beg you to use 'select count(*) from users' asap.
    //                                          - Also Me
    // I did it.
    //          - Also Me
    let c = get_user_count(km.pool()).await;
    let c = match c {
        Ok(p) => p,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error(ApiErrorE::Internal));
        }
    };

    let page = index_page(c as usize);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(page.into_string())
}

#[post("/login")]
pub async fn login_post(form: web::Form<LoginForm>, km: web::Data<KM>) -> impl Responder {
    match login_user(&km, form.into_inner()).await {
        Ok(u) => {
            let cookie = baker!(&km, "access_token", u, true);
            HttpResponse::SeeOther()
                .cookie(cookie)
                .append_header(("Location", "/mere"))
                .finish()
        }
        Err(e) => {
            tracing::warn!(error = %e, "register failed");
            HttpResponse::BadRequest().json(ApiResponse::error(e.into()))
        }
    }
}

#[get("/login")]
pub async fn login(km: web::Data<KM>) -> impl Responder {
    let page = login_page();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(page.into_string())
}

#[post("/register")]
pub async fn register_post(form: web::Form<RegisterForm>, km: web::Data<KM>) -> impl Responder {
    match register_user(&km, form.into_inner()).await {
        Ok(u) => HttpResponse::SeeOther()
            .append_header(("Location", "/login"))
            .finish(),
        Err(e) => {
            tracing::warn!(error = %e, "register failed");
            HttpResponse::BadRequest().json(ApiResponse::error(e.into()))
        }
    }
}

#[get("/register")]
pub async fn register(km: web::Data<KM>) -> impl Responder {
    let page = register_page();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(page.into_string())
}
