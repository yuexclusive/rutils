use rocket::serde::Deserialize;

#[derive(Deserialize, FromForm)]
pub struct Pagination {
    pub index: u64,
    pub size: u64,
}

impl Pagination {
    pub fn skip(&self) -> u64 {
        (self.index - 1) * self.size
    }

    pub fn take(&self) -> u64 {
        self.size
    }
}

#[derive(Deserialize)]
pub struct ID {
    pub id: i64,
}
