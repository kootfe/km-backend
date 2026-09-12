use actix_web::cookie::Cookie;

use crate::app::KM;

pub fn bake_cookie(km: &KM, name: impl Into<String>, value: impl Into<String>, http_only: bool) -> Cookie<'static> {
    Cookie::build(name.into(), value.into())
        .http_only(http_only)
        .secure(km.is_cookie_secure())
        .domain(km.domain())
        .finish()
        .into_owned()
}
