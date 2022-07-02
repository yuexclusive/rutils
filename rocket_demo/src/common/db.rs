use crate::config;
use sqlx::{Pool, Postgres, Transaction};
use std::result::Result;

pub type SqlResult<T, E = sqlx::Error> = Result<T, E>;

pub async fn conn() -> SqlResult<Pool<Postgres>> {
    sqlx::postgres::PgPoolOptions::new()
        .test_before_acquire(false)
        .connect(&config::CONFIG.pg.address)
        .await
}

pub async fn tran<'a>() -> SqlResult<Transaction<'a, Postgres>> {
    conn().await?.begin().await
}
