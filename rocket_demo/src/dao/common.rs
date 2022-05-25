use crate::config;
use std::result::Result;

use sqlx::{mysql::MySqlPool, Database, Error, Executor, MySql, Pool, Transaction};

pub type SqlResult<T, E = sqlx::Error> = Result<T, E>;

pub async fn pool() -> SqlResult<Pool<MySql>> {
    MySqlPool::connect(&config::CONFIG.mysql.address).await
}

pub async fn transaction<'a>() -> SqlResult<Transaction<'a, MySql>> {
    let a = pool().await?.begin().await;
    a
}
