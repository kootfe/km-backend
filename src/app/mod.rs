use sqlx::PgPool;

use crate::db::init_database;

pub mod trace;
pub mod server;
pub(crate) mod macors;

#[derive(Clone)]
pub struct KM { //Comes from app's name Kittens' Mayhem, KM
    db: PgPool,
    domain: String,
    secure_cookie: bool,
    jwt_secret: String,
}

impl KM {
    pub fn pool(&self) -> &PgPool {
        &self.db
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn is_cookie_secure(&self) -> bool {
        self.secure_cookie
    }

    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }

    pub async fn build_auto() -> anyhow::Result<Self> {
        let db_url = std::env::var("DATABASE_URL")?;
        let jwt_secret = std::env::var("JWT_SECRET")?;
        let db = init_database(&db_url).await?;
        let secure_cookie = std::env::var("COOKIE_SEC")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        Ok(Self {
            db,
            domain: "localhost".into(),
            secure_cookie,
            jwt_secret
        })
    }
}

