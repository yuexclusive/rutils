use std::marker::PhantomData;

use crate::common::db::SqlResult;
use crate::common::Pagination;
use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{self},
    Executor, Postgres,
};

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all(serialize = "snake_case",deserialize = "snake_case"))]
pub enum UserType {
    Normal,
    Admin,
    SuperAdmin,
}

// impl sqlx::postgres::PgHasArrayType for UserType {
//     fn array_type_info() -> sqlx::postgres::PgTypeInfo {
//         sqlx::postgres::PgTypeInfo::with_name("usertype")
//     }
// }

// impl From<UserType> for i32 {
//     fn from(t: UserType) -> Self {
//         match t {
//             UserType::Normal => 1,
//             UserType::Admin => 2,
//             UserType::SuperAdmin => 3,
//         }
//     }
// }

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub r#type: UserType, // 1. normal 2. admin 3.super admin
    pub email: String,
    pub name: Option<String>,
    pub salt: String,
    pub pwd: Option<String>,
    pub mobile: Option<String>,
    pub laston: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub struct Dao<'a, E>
where
    E: Executor<'a, Database = Postgres>,
{
    executor: E,
    _maker: PhantomData<&'a E>,
}

impl<'a, E> Dao<'a, E>
where
    E: Executor<'a, Database = Postgres>,
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
    E: Executor<'a, Database = Postgres>,
{
    pub async fn count(self) -> SqlResult<i64> {
        let res = sqlx::query!(
            r#"
select
    count(1) 
from
"user" u
where u.deleted_at is null
"#
        )
        .fetch_one(self.executor)
        .await?;
        Ok(res.count.unwrap())
    }

    pub async fn query(self, p: &Pagination) -> SqlResult<Vec<User>> {
        sqlx::query_as!(
            User,
            r#"
select
     id,
     "type" as "type!: UserType",
     email,
     "name",
     salt,
     pwd,
     mobile,
     laston,
     created_at,
     updated_at,
     deleted_at
from "user" u
where u.deleted_at is null
order by u.id
limit $1 offset $2
"#,
            p.take(),
            p.skip(),
        )
        .fetch_all(self.executor)
        .await
    }

    pub async fn get(self, id: i64) -> SqlResult<User> {
        let res = sqlx::query_as!(
            User,
            r#"
select
    id,
    "type" as "type!: UserType",
    email,
    "name",
    salt,
    pwd,
    mobile,
    laston,
    created_at,
    updated_at,
    deleted_at
from "user" 
where id = $1
            "#,
            id,
        )
        .fetch_one(self.executor)
        .await?;

        Ok(res)
    }

    pub async fn get_by_email(self, email: &str) -> SqlResult<User> {
        let res = sqlx::query_as!(
            User,
            r#"
select
    id,
    "type" as "type!: UserType",
    email,
    "name",
    salt,
    pwd,
    mobile,
    laston,
    created_at,
    updated_at,
    deleted_at
from "user" 
where email = $1
            "#,
            email,
        )
        .fetch_one(self.executor)
        .await?;

        Ok(res)
    }

    pub async fn insert(
        self,
        email: &str,
        salt: &str,
        pwd: &str,
        name: Option<&str>,
        mobile: Option<&str>,
    ) -> SqlResult<i64> {
        let created_at = chrono::Local::now().naive_local();
        let res = sqlx::query!(
            r#"insert into "user" (type,email,pwd,salt,name,mobile,created_at) values ($1,$2,$3,$4,$5,$6,$7) RETURNING id"#,
            UserType::Normal as UserType,
            email,
            pwd,
            salt,
            name,
            mobile,
            created_at,
        )
        .fetch_one(self.executor)
        .await?;

        Ok(res.id)
    }

    pub async fn delete(self, ids: &[i64]) -> SqlResult<u64> {
        let deleted_at = chrono::Local::now().naive_local();
        let res = sqlx::query!(
            r#"update "user" set deleted_at = $1 where id = ANY($2)"#,
            deleted_at,
            ids,
        )
        .execute(self.executor)
        .await?;

        Ok(res.rows_affected())
    }

    pub async fn update_pwd(self, id: i64, salt: &str, pwd: &str) -> SqlResult<u64> {
        let updated_at = chrono::Local::now().naive_local();
        let res = sqlx::query!(
            r#"update "user" set salt = $1, pwd = $2, updated_at=$3 where id = $4"#,
            salt,
            pwd,
            updated_at,
            id,
        )
        .execute(self.executor)
        .await?;

        Ok(res.rows_affected())
    }

    pub async fn update_laston(self, id: i64) -> SqlResult<u64> {
        let laston = chrono::Local::now().naive_local();
        let res = sqlx::query!(r#"update "user" set laston = $1 where id = $2"#, laston, id)
            .execute(self.executor)
            .await?;

        Ok(res.rows_affected())
    }
}
