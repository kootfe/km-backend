pub mod users;
use anyhow::Result;
use sqlx::PgPool;
pub async fn init_database(databse_url: &str) -> Result<PgPool> {

    let pool = PgPool::connect(databse_url).await?;
    Ok(pool)
}

