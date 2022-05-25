use crate::dao::common::pool;
use crate::dao::common::SqlResult;
use crate::dao::user as user_dao;

pub struct Service;

impl Service {
    pub fn new() -> Self {
        Self {}
    }
}

impl Service {
    pub async fn query(&self) -> SqlResult<Vec<user_dao::User>> {
        let res = user_dao::Dao::new().query(&pool().await?).await?;
        Ok(res)
    }
}
