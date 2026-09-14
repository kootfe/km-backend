use actix_web::{FromRequest, HttpResponse, ResponseError};
use core::fmt;
use std::future::{Ready, ready};

use crate::data::user::User;


#[derive(Debug)]
pub enum UserMiddlewareError {
    Forbidden,
    Unauthorized,
}

impl fmt::Display for UserMiddlewareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserMiddlewareError::Forbidden => write!(f, "Forbidden"),
            UserMiddlewareError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl ResponseError for UserMiddlewareError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            UserMiddlewareError::Forbidden => HttpResponse::Forbidden().finish(),
            UserMiddlewareError::Unauthorized => HttpResponse::Unauthorized().finish(),
        }
    }
}

impl FromRequest for User {
    type Error = UserMiddlewareError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let token = match req.cookie("") {
            Some(t) => t,
            None => return ready(Err(UserMiddlewareError::Unauthorized)),
        };
        let id = crate::http::jwt::get_user_uuid(token, secret);

        //Will remove this, if this is not here compiler will give more errors then 1 while dev time:
        Ok(_res)
    }
}
