use sqlx::Acquire;

use crate::dao::common::db::SqlResult;
use crate::dao::common::db::{conn, tran};
use crate::dao::user as user_dao;
use crate::model::common::Pagination;

pub struct Service;

impl Service {
    pub fn new() -> Self {
        Self {}
    }
}

impl Service {
    pub async fn query(&self, page: &Pagination) -> SqlResult<(Vec<user_dao::User>, i64)> {
        let data = user_dao::Dao::new(&conn().await?).query(page).await?;
        let count = user_dao::Dao::new(&conn().await?).count().await?;

        Ok((data, count))
    }

    pub async fn update(&self, id: i64, name: &str) -> SqlResult<u64> {
        user_dao::Dao::new(&conn().await?).update(id, name).await
    }

    pub async fn insert(&self, email: &str, role_id: i64) -> SqlResult<u64> {
        let mut tx = tran().await?;
        tx.begin().await?;
        let res = user_dao::Dao::new(&mut tx).insert(email).await?;
        user_dao::Dao::new(&mut tx)
            .insert_user_role_map(res as i64, role_id)
            .await?;
        tx.commit();
        Ok(res)
    }

    pub async fn delete(&self, id: i64) -> SqlResult<u64> {
        user_dao::Dao::new(&conn().await?).delete(id).await
    }
}
