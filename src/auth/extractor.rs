use crate::{
    auth::get_user_from_token,
    data::{AppState, KfResponse, User},
};
use actix_web::{FromRequest, HttpRequest, Result, web, HttpResponse};
use sqlx::PgPool;
use std::pin::Pin;
use std::future::Future;

pub struct AuthUser(pub User);

impl FromRequest for AuthUser {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone(); //Do NOT mind this.

        Box::pin(async move {
            let pool = req
                .app_data::<web::Data<PgPool>>()
                .ok_or_else(|| json_error("internal server error", 500, None))?;

            let state = req
                .app_data::<web::Data<AppState>>()
                .ok_or_else(|| json_error("internal server error", 500, None))?;

            let token = crate::auth::get_token_from_req(&req)
                .ok_or_else(|| json_error("missing token", 401, None))?;

            get_user_from_token(token, &state.jwt_secret, pool.get_ref())
                .await
                .map(AuthUser)
                .ok_or_else(|| json_error("invalid token", 401, Some(1)))
        })
    }
}

fn json_error(msg: &'static str, status: u16, code: Option<i32>) -> actix_web::Error {
    let body = match code {
        None => KfResponse::<()>::err(msg),
        Some(n) => KfResponse::<()>::err_c(n, msg),
    };

    let response = HttpResponse::build(
        actix_web::http::StatusCode::from_u16(status).unwrap()
    )
    .json(body);
    actix_web::error::InternalError::from_response(msg, response).into()
}
