use redis::ToRedisArgs;
use redis::{aio::Connection, AsyncCommands};
// use redis_encoding::ToRedisEncodingWrap;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use crate::config;

use super::BasicResult;

pub async fn conn() -> redis::RedisResult<Connection> {
    let client = redis::Client::open(config::CONFIG.redis.address.as_str())?;
    client.get_async_connection().await
}

pub async fn set<'a, K, V>(k: K, v: V) -> redis::RedisResult<()>
where
    K: redis::ToRedisArgs + Send + Sync + 'a,
    V: Serialize + ToRedisArgs + Send + Sync + 'a,
{
    conn().await?.set(k, v).await
}

pub async fn set_ex<'a, K, V>(k: K, v: V, seconds: usize) -> redis::RedisResult<()>
where
    K: redis::ToRedisArgs + Send + Sync + 'a,
    V: Serialize + ToRedisArgs + Send + Sync + 'a,
{
    conn().await?.set_ex(k, v, seconds).await
}

pub async fn get<'a, K, V>(k: K) -> BasicResult<V>
where
    K: redis::ToRedisArgs + Send + Sync + 'a,
    V: DeserializeOwned,
{
    let vec = conn().await?.get::<_, Vec<u8>>(k).await?;
    let res = bincode::deserialize::<V>(&vec)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::common::BasicResult;
    use redis_encoding_derive::ToRedisArgs;

    use super::*;
    #[derive(serde::Serialize, serde::Deserialize, ToRedisArgs, Debug)]
    struct Pancakes {
        name: String,
    }
    #[tokio::test]
    async fn test_redis() -> BasicResult<()> {
        set(
            "aa",
            Pancakes {
                name: "one and only".to_string(),
            },
        )
        .await?;

        let p = get::<_, Pancakes>("aa").await?;

        println!("res: {:?}", p);
        Ok(())
    }
}
