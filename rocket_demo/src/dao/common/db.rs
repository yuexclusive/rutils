use crate::config;
use sqlx::{mysql::MySqlPool, Database, MySql, Pool, Transaction};
use std::result::Result;

pub type SqlResult<T, E = sqlx::Error> = Result<T, E>;

pub async fn conn() -> SqlResult<Pool<MySql>> {
    sqlx::mysql::MySqlPoolOptions::new()
        .test_before_acquire(false)
        .connect(&config::CONFIG.mysql.address)
        .await
}

pub async fn tran<'a>() -> SqlResult<Transaction<'a, MySql>> {
    conn().await?.begin().await
}
