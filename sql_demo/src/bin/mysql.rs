#![allow(dead_code)]
#![allow(unused)]
extern crate tokio;

use sqlx::{mysql::MySqlPool, MySql, Pool};

#[tokio::main]
// or #[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    Ok(())
}

async fn pool() -> Result<Pool<MySql>, sqlx::Error> {
    let url_key = "DATABASE_URL";
    let conn_str = std::env::var(url_key).expect(&format!("failed to get env {}", url_key));
    MySqlPool::connect(&conn_str).await
}

#[derive(Debug)]
struct Cake {
    id: i32,
    name: String,
    cake_name: Option<String>,
}

async fn query() -> Result<Vec<Cake>, sqlx::Error> {
    sqlx::query_as!(Cake, "select f.id,f.name,c.name as cake_name from fruit f left join cake c on f.cake_id =c.id limit ? offset ?",10,0)
    .fetch_all(&pool().await?)
    .await
}

async fn first(id: i32) -> Result<Cake, sqlx::Error> {
    sqlx::query_as!(Cake, "select f.id,f.name,c.name as cake_name from fruit f left join cake c on f.cake_id =c.id where f.id = ?",id)
    .fetch_one(&pool().await?)
    .await
}

async fn last() -> Result<Cake, sqlx::Error> {
    sqlx::query_as!(Cake, "select f.id,f.name,c.name as cake_name from fruit f left join cake c on f.cake_id =c.id order by f.id desc limit 1")
    .fetch_one(&pool().await?)
    .await
}

async fn create() -> Result<u64, sqlx::Error> {
    let res = sqlx::query!("insert into fruit(name) values (?)", "test")
        .execute(&pool().await?)
        .await?;

    Ok(res.last_insert_id())
}

async fn update(id: i32, name: &str) -> Result<u64, sqlx::Error> {
    let res = sqlx::query!("update fruit set name=? where id in (?)", name, id)
        .execute(&pool().await?)
        .await?;

    Ok(res.rows_affected())
}

async fn delete(id: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query!("delete from fruit where id in (?)", id)
        .execute(&pool().await?)
        .await?;

    Ok(res.rows_affected())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{self, Duration, SystemTime};

    #[tokio::test]
    async fn test_first() -> Result<(), sqlx::Error> {
        let res = first(1).await?;

        println!("{:?}", res);

        Ok(())
    }

    #[tokio::test]
    async fn test_last() -> Result<(), sqlx::Error> {
        let res = last().await?;

        println!("{:?}", res);

        Ok(())
    }

    #[tokio::test]
    async fn test_create() -> Result<(), sqlx::Error> {
        let res = create().await?;
        println!("last_insert_id: {}", res);
        Ok(())
    }

    #[tokio::test]
    async fn test_update() -> Result<(), sqlx::Error> {
        let id = last().await?.id;
        let res = update(id, "Blueberry").await?;
        println!("rows_affected: {}", res);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> Result<(), sqlx::Error> {
        let id = last().await?.id;
        let res = delete(id).await?;
        println!("rows_affected: {}", res);
        Ok(())
    }

    #[tokio::test]
    async fn test_query_join() -> Result<(), sqlx::Error> {
        let now = SystemTime::now();
        let (f, q) = tokio::join!(first(1), query());

        println!("first: {:?}", f);
        println!("query: {:?}", q);

        println!("elapesd with join: {}", now.elapsed().unwrap().as_millis());
        Ok(())
    }

    #[tokio::test]
    async fn test_query() -> Result<(), sqlx::Error> {
        let now = SystemTime::now();
        let (f, q) = (first(1).await?, query().await);

        println!("first: {:?}", f);
        println!("query: {:?}", q);

        println!("elapesd: {}", now.elapsed().unwrap().as_millis());
        Ok(())
    }
}
