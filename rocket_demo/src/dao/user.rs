use super::common::pool;
use super::common::transaction;
use sqlx::{Error, Executor, MySql};

#[derive(Debug)]
pub struct User {
    id: i64,
    email: String,
    role_name: String,
}

pub struct Dao {
    table_name: String,
}

impl Dao {
    pub fn new() -> Self {
        Self {
            table_name: String::from("user"),
        }
    }

    pub async fn query<'a, E>(&self, e: E) -> Result<Vec<User>, Error>
    where
        E: Executor<'a, Database = MySql>,
    {
        sqlx::query_as!(
            User,
            "select u.id,u.email,r.name as role_name 
        from `user` u 
        join user_role_map urm on u.id =urm.user_id 
        join `role` r on urm.role_id  = r.id  
        limit ? offset ?",
            10,
            0
        )
        .fetch_all(e)
        .await
    }

    pub async fn first<'a, E>(&self, e: E, id: i64) -> Result<User, Error>
    where
        E: Executor<'a, Database = MySql>,
    {
        sqlx::query_as!(
            User,
            "select u.id,u.email,r.name as role_name 
            from `user` u 
            join user_role_map urm on u.id =urm.user_id 
            join `role` r on urm.role_id  = r.id
        where u.id = ?",
            id
        )
        .fetch_one(&pool().await?)
        .await
    }

    pub async fn last<'a, E>(&self, e: E) -> Result<User, Error>
    where
        E: Executor<'a, Database = MySql>,
    {
        let p = &pool().await?;
        sqlx::query_as!(
            User,
            "select u.id,u.email,r.name as role_name 
        from `user` u 
        join user_role_map urm on u.id =urm.user_id 
        join `role` r on urm.role_id  = r.id order by u.id desc limit 1"
        )
        .fetch_one(&pool().await?)
        .await
    }

    pub async fn insert<'a, E>(&self, e: E) -> Result<u64, Error>
    where
        E: Executor<'a, Database = MySql>,
    {
        let mut tr = transaction().await?;
        let id = uuid::Uuid::new_v4().to_string();
        let res = sqlx::query!("insert into user(email,salt) values (?,?)", "test", id)
            .execute(&mut tr)
            .await?;

        sqlx::query!(
            "insert into user_role_map(user_id,role_id) values (?,?)",
            res.last_insert_id(),
            1
        )
        .execute(&mut tr)
        .await?;

        tr.commit().await?;

        Ok(res.last_insert_id())
    }

    pub async fn update<'a, E>(&self, e: E, id: i64, name: &str) -> Result<u64, Error>
    where
        E: Executor<'a, Database = MySql>,
    {
        let res = sqlx::query!("update user set name=? where id in (?)", name, id)
            .execute(&pool().await?)
            .await?;

        Ok(res.rows_affected())
    }

    pub async fn delete<'a, E>(&self, e: E, id: i64) -> Result<u64, Error>
    where
        E: Executor<'a, Database = MySql>,
    {
        let res = sqlx::query!("delete from user where id in (?)", id)
            .execute(&pool().await?)
            .await?;

        Ok(res.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{self, Duration, SystemTime};

    #[tokio::test]
    async fn test_first() -> Result<(), Error> {
        let res = Dao::new().first(&pool().await?, 3).await?;

        println!("{:?}", res);

        Ok(())
    }

    #[tokio::test]
    async fn test_last() -> Result<(), Error> {
        let res = Dao::new().last(&pool().await?).await?;

        println!("{:?}", res);

        Ok(())
    }

    #[tokio::test]
    async fn test_insert() -> Result<(), Error> {
        let res = Dao::new().insert(&pool().await?).await?;
        println!("last_insert_id: {}", res);
        Ok(())
    }

    #[tokio::test]
    async fn test_update() -> Result<(), Error> {
        let mut tx = transaction().await?;
        let id = Dao::new().last(&mut tx).await?.id;
        let res = Dao::new().update(&mut tx, id, "Blueberry").await?;
        tx.commit().await?;
        println!("rows_affected: {}", res);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> Result<(), Error> {
        let mut tx = transaction().await?;
        let id = Dao::new().last(&mut tx).await?.id;
        let res = Dao::new().delete(&mut tx, id).await?;
        tx.commit().await?;
        println!("rows_affected: {}", res);
        Ok(())
    }

    #[tokio::test]
    async fn test_query_join() -> Result<(), Error> {
        let now = SystemTime::now();
        let e = &pool().await?;
        let dao = Dao::new();
        let f1 = dao.first(e, 3);
        let f2 = dao.query(e);
        let (f, q) = tokio::join!(f1, f2);

        println!("first: {:?}", f);
        println!("query: {:?}", q);

        println!("elapesd with join: {}", now.elapsed().unwrap().as_millis());
        Ok(())
    }

    #[tokio::test]
    async fn test_query() -> Result<(), Error> {
        let now = SystemTime::now();
        let (f, q) = (
            Dao::new().first(&pool().await?, 3).await?,
            Dao::new().query(&pool().await?).await,
        );

        println!("first: {:?}", f);
        println!("query: {:?}", q);

        println!("elapesd: {}", now.elapsed().unwrap().as_millis());
        Ok(())
    }
}
