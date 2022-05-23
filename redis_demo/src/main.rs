use redis::{aio::Connection, AsyncCommands};
use tokio::join;

async fn get_conn() -> redis::RedisResult<Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    client.get_async_connection().await
}

// #[redis::aio::tokio::AsyncCommands]
#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let _ = join!(test_1(), test_2());

    let result = redis::cmd("MGET")
        .arg(&["key1", "key2"])
        .query_async(&mut get_conn().await?)
        .await;
    assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));
    Ok(())
}

async fn test_1() -> redis::RedisResult<String> {
    redis::cmd("SET")
        .arg(&["key2", "bar"])
        .query_async(&mut get_conn().await?)
        .await
}

async fn test_2() -> redis::RedisResult<String> {
    get_conn().await?.set("key1", b"foo").await
}
