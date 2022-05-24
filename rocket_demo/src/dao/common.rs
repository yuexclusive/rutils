use crate::config;

use sqlx::{mysql::MySqlPool, Database, Error, Executor, MySql, Pool, Transaction};

pub async fn pool() -> Result<Pool<MySql>, Error> {
    MySqlPool::connect(&config::CONFIG.mysql.address).await
}

pub async fn transaction<'a>() -> Result<Transaction<'a, MySql>, Error> {
    let a = pool().await?.begin().await;
    a
}
