use crate::config;
use redis::{aio::Connection, AsyncCommands};
use tokio::join;

pub async fn conn() -> redis::RedisResult<Connection> {
    let client = redis::Client::open(config::CONFIG.redis.address.as_str())?;
    client.get_async_connection().await
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn test_1() -> redis::RedisResult<String> {
        redis::cmd("SET")
            .arg(&["key2", "bar"])
            .query_async(&mut conn().await?)
            .await
    }

    async fn test_2() -> redis::RedisResult<String> {
        conn().await?.set("key1", b"foo").await
    }

    #[tokio::test]
    async fn test_redis() -> redis::RedisResult<()> {
        let _ = join!(test_1(), test_2());

        let result = redis::cmd("MGET")
            .arg(&["key1", "key2"])
            .query_async(&mut conn().await?)
            .await;
        assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));
        Ok(())
    }
}
