use crate::{
    auth::get_user_from_token,
    data::{AppState, User},
};
use actix_web::{FromRequest, HttpRequest, Result, web};
use sqlx::PgPool;
use std::pin::Pin;

pub struct AuthUser(pub User);

impl FromRequest for AuthUser {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let pool = req
                .app_data::<web::Data<PgPool>>()
                .ok_or_else(|| actix_web::error::ErrorInternalServerError("no pool"))?;
            let state = req
                .app_data::<web::Data<AppState>>()
                .ok_or_else(|| actix_web::error::ErrorInternalServerError("no state"))?;

            let token = crate::auth::get_token_from_req(&req)
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("no token"))?;

            get_user_from_token(token, &state.jwt_secret, pool.get_ref())
                .await
                .map(AuthUser)
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("invalid token"))
        })
    }
}
