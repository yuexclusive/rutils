pub mod db;
pub mod error;
pub mod kafka;
pub mod meilisearch;
pub mod redis_util;

use rocket::serde::Deserialize;

// pub type BasicResult<T, E = Box<dyn Error>> = Result<T, E>;
pub type BasicResult<T, E = crate::common::error::ErrorKind> = Result<T, E>;

#[derive(Deserialize, FromForm)]
pub struct Pagination {
    pub index: i64,
    pub size: i64,
}

impl Pagination {
    pub fn skip(&self) -> i64 {
        self.index.checked_sub(1).unwrap_or(0) * self.size
    }

    pub fn take(&self) -> i64 {
        self.size.max(0)
    }
}

#[derive(Deserialize)]
pub struct ID {
    pub id: i64,
}
