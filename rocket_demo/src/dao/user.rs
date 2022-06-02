use std::marker::PhantomData;

use super::common::SqlResult;
use super::common::{pool, transaction};
use crate::model::common::Pagination;
use rocket::futures::FutureExt;
use sqlx::{Executor, MySql};

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub role_name: String,
}

pub struct Dao<'a, E>
where
    E: Executor<'a, Database = MySql>,
{
    executor: E,
    _maker: PhantomData<&'a E>,
}

impl<'a, E> Dao<'a, E>
where
    E: Executor<'a, Database = MySql>,
{
    pub fn new(e: E) -> Self {
        Self {
            executor: e,
            _maker: PhantomData,
        }
    }
}

impl<'a, E> Dao<'a, E>
where
    E: Executor<'a, Database = MySql>,
{
    pub async fn count(self) -> SqlResult<i64> {
        let res = sqlx::query!(
            "select count(1) as count from  
            `user` u 
            join user_role_map urm on u.id =urm.user_id 
            join `role` r on urm.role_id  = r.id "
        )
        .fetch_one(self.executor)
        .await?;
        Ok(res.count)
    }

    pub async fn query(self, p: &Pagination) -> SqlResult<Vec<User>> {
        sqlx::query_as!(
            User,
            "select u.id,u.email,r.name as role_name 
        from `user` u 
        join user_role_map urm on u.id =urm.user_id 
        join `role` r on urm.role_id  = r.id  
        limit ?,?",
            p.skip(),
            p.take(),
        )
        .map(|x| x)
        .fetch_all(self.executor)
        .await
    }
    pub async fn first(self, id: i64) -> SqlResult<User> {
        sqlx::query_as!(
            User,
            "select u.id,u.email,r.name as role_name 
            from `user` u 
            join user_role_map urm on u.id =urm.user_id 
            join `role` r on urm.role_id  = r.id
        where u.id = ?",
            id
        )
        .fetch_one(self.executor)
        .await
    }

    pub async fn last(self) -> SqlResult<User> {
        sqlx::query_as!(
            User,
            "select u.id,u.email,r.name as role_name 
        from `user` u 
        join user_role_map urm on u.id =urm.user_id 
        join `role` r on urm.role_id  = r.id order by u.id desc limit 1"
        )
        .fetch_one(self.executor)
        .await
    }

    pub async fn insert(self, email: &str) -> SqlResult<u64> {
        let id = uuid::Uuid::new_v4().to_string();
        let res = sqlx::query!("insert into user(email,salt) values (?,?)", email, id)
            .execute(self.executor)
            .await?;

        Ok(res.last_insert_id())
    }

    pub async fn insert_user_role_map(self, user_id: i64, role_id: i64) -> SqlResult<u64> {
        let res = sqlx::query!(
            "insert into user_role_map(user_id,role_id) values (?,?)",
            user_id,
            role_id,
        )
        .execute(self.executor)
        .await?;

        Ok(res.last_insert_id())
    }

    pub async fn update(self, id: i64, name: &str) -> SqlResult<u64> {
        let res = sqlx::query!("update user set name=? where id in (?)", name, id)
            .execute(self.executor)
            .await?;

        Ok(res.rows_affected())
    }

    pub async fn delete(self, id: i64) -> SqlResult<u64> {
        let res = sqlx::query!("delete from user where id in (?)", id)
            .execute(self.executor)
            .await?;

        Ok(res.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{self, Duration, SystemTime};

    #[tokio::test]
    async fn test_first() -> SqlResult<()> {
        let res = Dao::new(&pool().await?).first(3).await?;

        println!("{:?}", res);

        Ok(())
    }

    #[tokio::test]
    async fn test_last() -> SqlResult<()> {
        let res = Dao::new(&pool().await?).last().await?;

        println!("{:?}", res);

        Ok(())
    }

    #[tokio::test]
    async fn test_insert() -> SqlResult<()> {
        let mut tx = transaction().await?;
        let res = Dao::new(&mut tx).insert("test").await?;
        Dao::new(&mut tx)
            .insert_user_role_map(res as i64, 1)
            .await?;
        println!("last_insert_id: {}", res);
        tx.commit();
        Ok(())
    }

    #[tokio::test]
    async fn test_update() -> SqlResult<()> {
        let mut tx = transaction().await?;
        let id = Dao::new(&mut tx).last().await?.id;
        let res = Dao::new(&mut tx).update(id, "Blueberry").await?;
        tx.commit().await?;
        println!("rows_affected: {}", res);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> SqlResult<()> {
        let mut tx = transaction().await?;
        let id = Dao::new(&mut tx).last().await?.id;
        let res = Dao::new(&mut tx).delete(id).await?;
        tx.commit().await?;
        println!("rows_affected: {}", res);
        Ok(())
    }

    #[tokio::test]
    async fn test_query_join() -> SqlResult<()> {
        let now = SystemTime::now();
        let e = &pool().await?;
        let f1 = Dao::new(e).first(3);
        let f2 = Dao::new(e).query(&Pagination { index: 1, size: 10 });
        let (f, q) = tokio::join!(f1, f2);

        println!("first: {:?}", f);
        println!("query: {:?}", q);

        println!("elapesd with join: {}", now.elapsed().unwrap().as_millis());
        Ok(())
    }

    #[tokio::test]
    async fn test_query() -> SqlResult<()> {
        let now = SystemTime::now();
        let e = &pool().await?;
        let (f, q) = (
            Dao::new(e).first(3).await?,
            Dao::new(e).query(&Pagination { index: 1, size: 10 }).await,
        );

        println!("first: {:?}", f);
        println!("query: {:?}", q);

        println!("elapesd: {}", now.elapsed().unwrap().as_millis());
        Ok(())
    }
}
