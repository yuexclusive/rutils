use crate::config;
use sqlx::{mysql::MySqlPool, Database, MySql, Pool, Transaction};
use std::result::Result;

pub type SqlResult<T, E = sqlx::Error> = Result<T, E>;

pub async fn pool() -> SqlResult<Pool<MySql>> {
    MySqlPool::connect(&config::CONFIG.mysql.address).await
}

pub async fn transaction<'a>() -> SqlResult<Transaction<'a, MySql>> {
    let a = pool().await?.begin().await;
    a
}
