use sqlx::postgres::PgPoolOptions;
use sqlx::error::Error;
use sqlx::{Pool, Postgres};

pub async fn create_pool(n : u32) -> Result<sqlx::Pool<sqlx::Postgres>, Error> {
    let pool = PgPoolOptions::new()
        .max_connections(n)
        .connect("postgres://postgres:admin@localhost:5432/rps").await?;
    Ok(pool)
}

pub async fn create_schema(pool : Pool<Postgres>) {

}