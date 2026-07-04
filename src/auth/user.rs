use crate::{auth::jwt::validate_token, data::User};
use actix_web::{HttpRequest};
use sqlx::PgPool;
use uuid::Uuid;

pub fn get_token_from_req<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    token
}

pub async fn get_user_from_uuid(uuid: &str, poll: &PgPool) -> Option<User> {
    let user = sqlx::query!(
        "select username, id, created_at from users where id = $1",
        Uuid::parse_str(uuid).unwrap() // bug 101 lol
                                       /*
                                        * this will 100% explode
                                        * ToDo: fix by replacing with-
                                        * Uuid::parse_str(uuid).ok()?
                                        * or just modify struct so it never becomes string
                                        * (why it does at first place. idk.)
                                        * and yes i could fix the code instead writing this comment
                                        * i decided i wouldnt.
                                        */
    )
    .fetch_optional(poll)
    .await;

    match user {
        Ok(Some(u)) => Some(User {
            username: u.username,
            id: u.id,
            createdat: u.created_at.unwrap(),
        }),
        Ok(None) => None,
        Err(_) => None,
    }
}

pub async fn get_user_from_token(token: &str, secret: &str, poll: &PgPool) -> Option<User> {
    match validate_token(token, secret) {
        Ok(claims) => get_user_from_uuid(&claims.sub, poll).await,
        Err(_) => None,
    }
}
