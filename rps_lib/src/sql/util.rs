use sqlx::postgres::PgPoolOptions;
use sqlx::error::Error;
use sqlx::{Pool, Postgres};
use sqlx::query;

pub async fn create_pool(n : u32) -> Result<sqlx::Pool<sqlx::Postgres>, Error> {
    let pool = PgPoolOptions::new()
        .max_connections(n)
        .connect("postgres://postgres:admin@localhost:5432/rps").await?;
    Ok(pool)
}

pub async fn insert_user(pool : Pool<Postgres>, name : String, wins: i32, losses : i32) -> Result<(), Error> {
    query("INSERT INTO client.user VALUES ($1, $2, $3)")
        .bind(name)
        .bind(wins)
        .bind(losses)
        .execute(&pool)
        .await;
}