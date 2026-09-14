use anyhow::Result;
use sqlx::{PgPool, pool};
use uuid::Uuid;

use crate::data::user::{User, UserAuth};

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>> {
    let users = sqlx::query_as!(User, "select id, username from users")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

pub async fn get_user_count(pool: &PgPool) -> Result<i64> {
    let row = sqlx::query!("select count(*) as count from users")
        .fetch_one(pool)
        .await?;
    Ok(row.count.unwrap_or(0))
}

pub async fn get_user_by_name(pool: &PgPool, name: &str) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        "select id, username from users where username = $1",
        name
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_auth_user_by_name(pool: &PgPool, name: &str) -> Result<UserAuth> {
    let user = sqlx::query_as!(
        UserAuth,
        "select id, username, pass_hash, jwt_v from users where username = $1",
        name
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn insert_user(pool: &PgPool, name: &str, hash: &str) -> Result<User> {
    let id = Uuid::new_v4();

    let user = sqlx::query_as!(
        User,
        "insert into users (id, username, pass_hash) values ($1, $2, $3) returning id, username",
        id,
        name,
        hash
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        "select id, username from users where id = $1",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_auth_user_by_id(pool: &PgPool, id: Uuid) -> Result<UserAuth> {
    let user = sqlx::query_as!(
        UserAuth,
        "select id, username, pass_hash, jwt_v from users where id = $1",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
